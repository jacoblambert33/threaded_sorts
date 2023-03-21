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
