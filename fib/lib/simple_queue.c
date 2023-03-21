#include "simple_queue.h"

#include <stdlib.h>

node_t *head = NULL;
node_t *tail = NULL;

void enqueue(long *input) {
  node_t *new = malloc(sizeof(node_t));
  new->input = input;
  new->next = NULL;

  if (tail == NULL) {
    head = new;
  } else {
    tail->next = new;
  }
  tail = new;
}

long *dequeue() {
  if (head == NULL)
    return NULL;
  else {
    long *ans = head->input;
    node_t *tmp = head->next;
    head = head->next;
    if (head == NULL) tail = NULL;
    free(tmp);
    return ans;
  }
}


/* 
NOTES: considering a solution where a static number of threads are created and reused. a threadpool. 

i can't let my threads die. they are doing the work. so i can't return from
these threads, i.e., i can't use join. so - try to use the serial fib function inside the threading function. but then i have no way to return the value to the calling function. 
for academic purposes, can i extend the queue to hold the return value and the thread itself. the negative consequences of this approach is lots of extra careful work. what about a cutoff instead of a queue? let's
try a cutoff.

*/


