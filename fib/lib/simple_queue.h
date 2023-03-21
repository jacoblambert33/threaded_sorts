#ifndef SIMPLE_QUEUE_H
#define SIMPLE_QUEUE_H

typedef struct node {
  struct node *next;
  long *input;
} node_t;

// typedef struct node node_t;

void enqueue(long *input);

long *dequeue();

#endif  // SIMPLE_QUEUE_H
