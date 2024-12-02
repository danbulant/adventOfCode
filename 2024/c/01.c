#include <stdlib.h> 
#include <stdio.h>
#include <string.h>

#define usize unsigned long

// A Vector type
struct DynSlice {
    void* data;
    usize size;
    usize capacity;
    usize size_of;
};

// Constructs a new dyn slice, a vector type
// default capacity is 0 and no backing data is allocated, use DS_grow
struct DynSlice DS_create(usize size_of) {
    struct DynSlice slice;
    slice.data = NULL;
    slice.size = 0;
    slice.capacity = 0;
    slice.size_of = size_of;
    return slice;
}

// Exit with message and error code 1
void panic(char* msg) {
    printf("%s\n", msg);
    exit(1);
}

// Grows slice to at least the given size (nearest power of 2 higher)
// panics if malloc fails
void DS_grow(struct DynSlice* slice, usize newcap) {
    usize cap = slice->capacity;
    usize size = slice->size;
    printf("Growing %li %li\n", cap, size);

    void* dataptr = slice->data;
    
    if(cap == 0) cap = 1;
    while(cap < newcap) {
        cap <<= 1;
    }

    void* newdataptr = malloc(cap * slice->size_of);
    if(newdataptr == NULL) {
        panic("DS malloc failed during grow");
    }
    if(dataptr) {
        memcpy(newdataptr, dataptr, size * slice->size_of);
        free(dataptr);
    }
    slice->capacity = cap;
    slice->data = newdataptr;
}

void DS_debugprint(struct DynSlice* slice) {
    printf("dynslice, size %li, cap %li, size of %li", slice->size, slice->capacity, slice->size_of);
}

// Pushes a new object. Accepts a pointer to said object. Copies. 
// returns index of pushed data
// assumes size_of from slice. Take care of types lol.
usize DS_push(struct DynSlice* slice, void* ptr) {
    usize cap = slice->capacity;
    usize size = slice->size;
    if(cap <= size) {
        DS_grow(slice, cap+1);
    }
    memcpy(slice->data + slice->size * slice->size_of, ptr, slice->size_of);
    slice->size += 1;
    return size;
}

// Extends a slice from another array-like object
void DS_extend(struct DynSlice* slice, void* ptr, usize count) {
    if(slice->capacity < slice->size + count) DS_grow(slice, slice->size + count);
    printf("extending %li\n", count);
    DS_debugprint(slice);
    memcpy(slice->data + slice->size * slice->size_of, ptr, slice->size_of * count);
    slice->size += count;
}

// Extends DynSlice from another DynSlice, checking their sizes for (partial) memory safety
void DS_extend_slice(struct DynSlice* target, struct DynSlice* src) {
    if(target->size_of != src->size_of) {
        panic("DS invalid extend_slice, different sizes");
    }
    DS_extend(target, src->data, src->size);
}

// Extends a slice from string. Checks size_of is correct, and calls strlen
void DS_extend_string(struct DynSlice* slice, void* ptr) {
    if(slice->size_of != sizeof(char)) panic("DS extend string on a non-char slice");
    usize len = strlen(ptr);
    DS_extend(slice, ptr, len);
}

// Drops slice in place - resets to empty
void DS_drop(struct DynSlice* slice) {
    slice->capacity = 0;
    slice->size = 0;
    free(slice->data);
}

// Gets pointer to a single element, checking it's size
void* DS_get(struct DynSlice* slice, usize i) {
    if(slice->size >= i) {
        panic("DS Get out of bounds");
    }
    return slice->data + i * slice->size_of;
}

int main() {
    // FILE* file = fopen("input.txt", "r");
    setvbuf(stdout, NULL, _IONBF, 0);
    struct DynSlice example = DS_create(sizeof(char));
    DS_extend_string(&example, "Hello, ");
    DS_extend_string(&example, "world!");
    DS_push(&example, '\0');
    printf("%s\n", (char*)example.data);
    DS_drop(&example);
}