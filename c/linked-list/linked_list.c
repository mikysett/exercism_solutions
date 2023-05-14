#include "linked_list.h"
#include <stdlib.h>
#include <assert.h>

typedef struct list_node {
   struct list_node *prev, *next;
   ll_data_t data;
} t_list_node;

typedef struct list {
   struct list_node *first, *last;
} t_list;

t_list *list_create(void) {
   t_list *new_list = malloc(sizeof(t_list));
   assert(new_list);
   new_list->first = NULL;
   new_list->last = NULL;
   return (new_list);
}

size_t list_count(const struct list *list) {
   assert(list);
   size_t size = 0;
   t_list_node *curr = list->first;
   while (curr) {
      curr = curr->next;
      size++;
   }
   return size;
}

void list_push(struct list *list, ll_data_t item_data) {
   t_list_node *new_node = malloc(sizeof(t_list_node));
   assert(list && new_node);
   new_node->next = NULL;
   new_node->prev = list->last;
   new_node->data = item_data;

   if (!list->first) {
      list->first = new_node;
   } else {
      list->last->next = new_node;
   }
   list->last = new_node;
}

ll_data_t list_pop(struct list *list) {
   assert(list && list->last);
   t_list_node *last_node = list->last;
   ll_data_t data = last_node->data;
   list->last = list->last->prev;
   if (list->last) {
      list->last->next = NULL;
   } else {
      list->first = NULL;
   }
   free(last_node);
   return data;
}

void list_unshift(struct list *list, ll_data_t item_data) {
   t_list_node *new_node = malloc(sizeof(t_list_node));
   assert(list && new_node);
   new_node->next = list->first;
   new_node->prev = NULL;
   new_node->data = item_data;

   if (!list->last) {
      list->last = new_node;
   } else {
      list->first->prev = new_node;
   }
   list->first = new_node;
}

ll_data_t list_shift(struct list *list) {
   assert(list && list->first);
   t_list_node *first_node = list->first;
   ll_data_t data = first_node->data;
   list->first = list->first->next;
   if (list->first) {
      list->first->prev = NULL;
   } else {
      list->last = NULL;
   }
   free(first_node);
   return data;
}

void list_delete(struct list *list, ll_data_t data) {
   assert(list);
   t_list_node *curr = list->first;
   while (curr) {
      if (curr->data == data) {
         if (curr == list->first) {
            list_shift(list);
         } else if (curr == list->last) {
            list_pop(list);
         } else {
            curr->prev->next = curr->next;
            curr->next->prev = curr->prev;
            free(curr);
         }
         break;
      }
      curr = curr->next;
   }
}

void list_destroy(struct list *list) {
   assert(list);
   t_list_node *curr = list->first;
   while (curr) {
      t_list_node *next = curr->next;
      free(curr);
      curr = next;
   }
   free(list);
}
