#include "threadpool.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void *tpool_thread(void *tpoolv)
// void tpool_thread(tpool_t tpool)
{
  // tpool_t tpool = *((tpool_t *)tpoolv);
  tpool_t tpool = malloc(sizeof(tpool));
  // tpool = *((tpool_t *)tpoolv);
  tpool = (tpool_t *)tpoolv;
  // tpool_t tpool = (tpool_t *)tpoolv;
  // WRONGtpool_t *tpool = (tpool_t *)tpoolv;

  tpool_work_t *my_workp;

  for (;;) {
    pthread_mutex_lock(&(tpool->queue_lock));
    while ((tpool->cur_queue_size == 0) && (!tpool->shutdown)) {
      pthread_cond_wait(&(tpool->queue_not_empty), &(tpool->queue_lock));
    }

    if (tpool->shutdown) {
      pthread_mutex_unlock(&(tpool->queue_lock));
      pthread_exit(NULL);
    }

    my_workp = tpool->queue_head;
    tpool->cur_queue_size--;
    if (tpool->cur_queue_size == 0)
      tpool->queue_head = tpool->queue_tail = NULL;
    else
      tpool->queue_head = my_workp->next;

    if ((!tpool->do_not_block_when_full) &&
        (tpool->cur_queue_size == (tpool->max_queue_size - 1)))
      pthread_cond_broadcast(&(tpool->queue_not_full));

    if (tpool->cur_queue_size == 0) pthread_cond_signal(&(tpool->queue_empty));
    pthread_mutex_unlock(&(tpool->queue_lock));
    (*(my_workp->routine))(my_workp->arg);
    free(my_workp);
  }
}

void tpool_init(tpool_t *tpoolp, int num_worker_threads, int max_queue_size,
                int do_not_block_when_full) {
  int i, rtn;
  tpool_t tpool;

  /* allocate a pool data structure */
  if ((tpool = (tpool_t)malloc(sizeof(struct tpool))) == NULL)
    perror("malloc"), exit(-1);

  /* initialize the fields */
  tpool->num_threads = num_worker_threads;
  tpool->max_queue_size = max_queue_size;
  tpool->do_not_block_when_full = do_not_block_when_full;
  if ((tpool->threads =
           (pthread_t *)malloc(sizeof(pthread_t) * num_worker_threads)) == NULL)
    perror("malloc"), exit(-1);
  tpool->cur_queue_size = 0;
  tpool->queue_head = NULL;
  tpool->queue_tail = NULL;
  tpool->queue_closed = 0;
  tpool->shutdown = 0;
  if ((rtn = pthread_mutex_init(&(tpool->queue_lock), NULL)) != 0)
    fprintf(stderr, "pthread_mutex_init %s", strerror(rtn)), exit(-1);
  if ((rtn = pthread_cond_init(&(tpool->queue_not_empty), NULL)) != 0)
    fprintf(stderr, "pthread_cond_init %s", strerror(rtn)), exit(-1);
  if ((rtn = pthread_cond_init(&(tpool->queue_not_full), NULL)) != 0)
    fprintf(stderr, "pthread_cond_init %s", strerror(rtn)), exit(-1);
  if ((rtn = pthread_cond_init(&(tpool->queue_empty), NULL)) != 0)
    fprintf(stderr, "pthread_cond_init %s", strerror(rtn)), exit(-1);

  /* create threads */
  for (i = 0; i != num_worker_threads; i++) {
    /*pthread_t t = tpool->threads[i];
if ((rtn = pthread_create( &t, */ //UNNECESSARY
    if ((rtn = pthread_create(&(tpool->threads[i]), NULL, tpool_thread,
                              tpool)) != 0)
      //(void *)tpool)) != 0)
      fprintf(stderr, "pthread_create %d", rtn), exit(-1);
  }

  *tpoolp = tpool;
}

int tpool_destroy(tpool_t tpool, int finish) {
  int i, rtn;
  tpool_work_t *cur_nodep;

  if ((rtn = pthread_mutex_lock(&(tpool->queue_lock))) != 0)
    fprintf(stderr, "pthread_mutex_lock %d", rtn), exit(-1);

  /* Is a shutdown already in progress? */
  if (tpool->queue_closed || tpool->shutdown) {
    if ((rtn = pthread_mutex_unlock(&(tpool->queue_lock))) != 0)
      fprintf(stderr, "pthread_mutex_unlock %d", rtn), exit(-1);
    return 0;
  }

  tpool->queue_closed = 1;

  /* If the finish flag is set, wait for workers to drain queue */
  if (finish == 1) {
    while (tpool->cur_queue_size != 0) {
      if ((rtn = pthread_cond_wait(&(tpool->queue_empty),
                                   &(tpool->queue_lock))) != 0)
        fprintf(stderr, "pthread_cond_wait %d", rtn), exit(-1);
    }
  }

  tpool->shutdown = 1;

  if ((rtn = pthread_mutex_unlock(&(tpool->queue_lock))) != 0)
    fprintf(stderr, "pthread_mutex_unlock %d", rtn), exit(-1);

  /* Wake up any workers so they recheck shutdown flag */
  if ((rtn = pthread_cond_broadcast(&(tpool->queue_not_empty))) != 0)
    fprintf(stderr, "pthread_cond_broadcast %d", rtn), exit(-1);
  if ((rtn = pthread_cond_broadcast(&(tpool->queue_not_full))) != 0)
    fprintf(stderr, "pthread_cond_broadcast %d", rtn), exit(-1);

  /* Wait for workers to exit */
  for (i = 0; i < tpool->num_threads; i++) {
    if ((rtn = pthread_join(tpool->threads[i], NULL)) != 0)
      fprintf(stderr, "pthread_join  %d", rtn), exit(-1);
  }

  /* Now free pool structures */
  free(tpool->threads);
  while (tpool->queue_head != NULL) {
    cur_nodep = tpool->queue_head->next;
    tpool->queue_head = tpool->queue_head->next;
    free(cur_nodep);
  }
  free(tpool);
  return 0;
}

int tpool_add_work(tpool_t tpool, void *routine, void *arg) {
  tpool_work_t *workp;
  pthread_mutex_lock(&tpool->queue_lock);

  if ((tpool->cur_queue_size == tpool->max_queue_size) &&
      tpool->do_not_block_when_full) {
    pthread_mutex_unlock(&tpool->queue_lock);
    return -1;
  }
  while ((tpool->cur_queue_size == tpool->max_queue_size) &&
         (!(tpool->shutdown || tpool->queue_closed))) {
    pthread_cond_wait(&tpool->queue_not_full, &tpool->queue_lock);
  }

  if (tpool->shutdown || tpool->queue_closed) {
    pthread_mutex_unlock(&tpool->queue_lock);
    return -1;
  }

  /* allocate work structure */
  workp = (tpool_work_t *)malloc(sizeof(tpool_work_t));
  workp->routine = routine;
  workp->arg = arg;
  workp->next = NULL;
  if (tpool->cur_queue_size == 0) {
    tpool->queue_tail = tpool->queue_head = workp;
    pthread_cond_broadcast(&tpool->queue_not_empty);
  } else {
    (tpool->queue_tail)->next = workp;
    tpool->queue_tail = workp;
  }
  tpool->cur_queue_size++;
  pthread_mutex_unlock(&tpool->queue_lock);
  return 1;
}
