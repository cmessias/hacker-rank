#include <stdio.h>
#include <stdint.h>

struct heap {
    uint32_t heap_size;
    uint32_t capacity;
    int32_t *array;
};

void heapify_up(struct heap *h, size_t child);
void heapify_down(struct heap *h, size_t parent);
void swap(int32_t *a, int32_t *b);

size_t parent_node(size_t i) {return (i - 1) / 2; }
size_t left_node(size_t i) { return 2 * i + 1; }
size_t right_node(size_t i) { return 2 * i + 2; }

struct heap make_heap(int32_t array[], uint32_t n)
{
        struct heap h = {
                .heap_size = n,
                .capacity = n,
                .array = array
        };

        size_t last_non_leaf = (n / 2) - 1;

        for (size_t i = last_non_leaf; i--;) {
                heapify_down(&h, i);
        }

        return h;
}

int32_t peek(struct heap h) { return h.array[0]; }

int32_t pop(struct heap *h)
{
        if (h->heap_size == 0) {
                return -1;
        }

        int32_t top = h->array[0];
         h->array[0] = h->array[h->heap_size - 1];
         h->heap_size--;
         heapify_down(h, 0);
         return top;
}

void heapify_down(struct heap *h, size_t parent)
{
        size_t largest = parent;
        size_t left = left_node(parent);
        size_t right = right_node(parent);

        if (left < h->heap_size && h->array[left] > h->array[largest]) {
                largest = left;
        }

        if (right < h->heap_size && h->array[right] > h->array[largest]) {
                largest = right;
        }

        if (largest != parent) {
                swap(&h->array[parent], &h->array[largest]);
                heapify_down(h, largest);
        }
}

void insert(struct heap *h, int32_t k)
{
        if (h->heap_size == h->capacity) {
                return;
        }

        h->array[h->heap_size] = k;
        heapify_up(h, h->heap_size);
        h->heap_size++;
}

void heapify_up(struct heap *h, size_t child)
{
        while (child > 0 && h->array[child] > h->array[parent_node(child)]) {
                swap(&h->array[child], &h->array[parent_node(child)]);
                child = parent_node(child);
        }
}

void swap(int32_t *a, int32_t *b)
{
        int32_t tmp = *a;
        *a = *b;
        *b = tmp;
}

void print_heap(struct heap *h)
{
        for (size_t i = 0; i < h->heap_size; ++i) {
                printf("%d ", h->array[i]);
        }
        printf("\n");
}

int main()
{
  int32_t arr[] = {1, 3, 5, 4};
  uint32_t n = sizeof(arr) / sizeof(arr[0]);

  struct heap h = make_heap(arr, n);
  print_heap(&h);
  pop(&h);
  print_heap(&h);
  pop(&h);
  print_heap(&h);
  pop(&h);
  print_heap(&h);
  insert(&h, 99);
  print_heap(&h);
  pop(&h);
  print_heap(&h);
  pop(&h);
  print_heap(&h);
  insert(&h, 10);
  print_heap(&h);
  insert(&h, 11);
  print_heap(&h);
}