#ifndef LHTML_SERIALIZER_H
#define LHTML_SERIALIZER_H

#include "tokenizer.h"

typedef void (*lhtml_string_callback_t)(lhtml_string_t string, void *extra);

typedef struct {
    lhtml_string_callback_t writer;
    bool compact;
} lhtml_serializer_options_t;

typedef struct {
    lhtml_token_handler_t handler; // needs to be the first one
    lhtml_serializer_options_t options;
} lhtml_serializer_state_t;

void lhtml_serializer_inject(lhtml_state_t *tokenizer, lhtml_serializer_state_t *state, const lhtml_serializer_options_t *options);

#endif