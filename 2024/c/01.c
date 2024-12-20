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
struct DynSlice DS_create(const usize size_of) {
    struct DynSlice slice;
    slice.data = NULL;
    slice.size = 0;
    slice.capacity = 0;
    slice.size_of = size_of;
    return slice;
}

// Exit with message and error code 1
void panic(const char* msg) {
    printf("%s\n", msg);
    exit(1);
}

// Grows slice to at least the given size (nearest power of 2 higher)
// panics if malloc fails
void DS_grow(struct DynSlice* slice, usize newcap) {
    usize cap = slice->capacity;
    usize size = slice->size;

    if(cap >= newcap) return;

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

void DS_debugprint(const struct DynSlice* slice) {
    printf("dynslice, size %li, cap %li, size of %li\n", slice->size, slice->capacity, slice->size_of);
}

void* DS_iptr(struct DynSlice* slice, usize index) {
    return slice->data + index * slice->size_of;
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
    memcpy(DS_iptr(slice, slice->size), ptr, slice->size_of);
    slice->size += 1;
    return size;
}

// Extends a slice from another array-like object
void DS_extend(struct DynSlice* slice, void* ptr, usize count) {
    DS_grow(slice, slice->size + count);
    memcpy(DS_iptr(slice, slice->size), ptr, slice->size_of * count);
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
void* DS_get(const struct DynSlice* slice, const usize i) {
    if(slice->size <= i) {
        panic("DS Get out of bounds");
    }
    return slice->data + i * slice->size_of;
}

usize UZERO = 0;

struct DynSlice DS_line_from_file(FILE* input) {
    int BUF_SIZE = 1024;
    struct DynSlice data = DS_create(sizeof(char));
    long int pos = ftell(input);
    
    char buf[BUF_SIZE+1];
    buf[BUF_SIZE] = 0;
    while(1) {
        const usize code = fread(buf, sizeof(buf[0]), BUF_SIZE, input);
        if(code != BUF_SIZE && ferror(input)) {
            panic("DS line file error");
        }
        buf[code] = 0; // strchr reads until 0 byte, this can guarantee it
        const char* result = strchr(buf, '\n');
        if(result != NULL) {
            const usize num = result - buf;
            if(num < code) {
                DS_extend(&data, buf, num);
                DS_push(&data, &UZERO);
                pos += num + 1;
                fseek(input, pos, SEEK_SET);
                return data;
            }
        }
        DS_extend(&data, buf, code);
        // error state checked before, this means EOF
        if(code != BUF_SIZE) {
            DS_push(&data, &UZERO);
            return data;
        }
    }
}

// Swaps two indexes in place, using last element of vector (may grow the vector and cause alloc)
void DS_swap(struct DynSlice* slice, usize i, usize j) {
    DS_grow(slice, slice->size + 1);
    memcpy(DS_iptr(slice, slice->size), DS_iptr(slice, i), slice->size_of);
    memcpy(DS_iptr(slice, i), DS_iptr(slice, j), slice->size_of);
    memcpy(DS_iptr(slice, j), DS_iptr(slice, slice->size), slice->size_of);
}

void DS_intdebugprint(struct DynSlice* slice) {
    if(slice->size_of != sizeof(int)) panic("using int debug print on non-int");

    DS_debugprint(slice);
    printf("values: ");
    for(usize i = 0; i < slice->size; i += 1) {
        printf("%i ", *(int*)DS_get(slice, i));
    }
    printf("\n");
}


// Sorts the array between left and right
// Uses a compare function. Returning -1 means the left number is smaller, 0 =, 1 left > right
// quicksort impl based on https://www.youtube.com/watch?v=Hoixgm4-P4M
void DS_sort_sliced(struct DynSlice* slice, int (*compare)(void*, void*), usize left, usize right) {
    if(right <= left) return;

    usize pivot = left;
    usize len = right - left;

    DS_swap(slice, pivot, right);
    pivot = right;

    while(1) {
        usize itemFromLeft = left;
        while(compare(DS_get(slice, itemFromLeft), DS_get(slice, pivot)) < 0)
            itemFromLeft += 1;

        usize itemFromRight = right - 1;
        while(compare(DS_get(slice, itemFromRight), DS_get(slice, pivot)) > 0)
            itemFromRight -= 1;

        if(itemFromLeft < itemFromRight) {
            DS_swap(slice, itemFromLeft, itemFromRight);
        } else {
            DS_swap(slice, itemFromLeft, pivot);
            pivot = itemFromLeft;
            DS_intdebugprint(slice);
            break;
        }
    }
}

// Sort the array using Quicksort
// Uses a compare function. Returning -1 means the left number is smaller, 0 =, 1 left > right
void DS_sort(struct DynSlice* slice, int (*compare)(void*, void*)) {
    DS_sort_sliced(slice, compare, 0, slice->size);
}

int intcompare(int* a, int* b) {
    if(*a > *b) return 1;
    if(*a == *b) return 0;
    if(*a < *b) return -1;
}

int main(int argc, char *argv[]) {
    setvbuf(stdout, NULL, _IONBF, 0);
    const char* loc = argv[1];
    printf("file %s\n", loc);
    FILE* input = fopen(loc, "r");
    
    struct DynSlice left = DS_create(sizeof(int));
    struct DynSlice right = DS_create(sizeof(int));

    while(!feof(input)) {
        struct DynSlice line = DS_line_from_file(input);
        
        // line is in a format of int   int
        // find space, add 2 to it. Then 0 - space is one int, space+3 - end is the other one.
        // we do this by inserting a null byte to effectively split into two strings

        char* spaceptr = strchr(line.data, ' ');
        *spaceptr = '\0';
        spaceptr += 3;

        int first = atoi(line.data);
        DS_push(&left, &first);
        int second = atoi(spaceptr);
        DS_push(&left, &first);

        DS_drop(&line);
    }

    DS_intdebugprint(&left);
    DS_sort(&left, intcompare);

    fclose(input);
    DS_drop(&left);
    DS_drop(&right);
}