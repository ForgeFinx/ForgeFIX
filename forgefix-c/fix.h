#ifndef _FIX_H
#define _FIX_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "fix_fields.h"

typedef enum c_fix_error {
  C_FIX_ERROR_OK,
  C_FIX_ERROR_IO_ERROR,
  C_FIX_ERROR_SESSION_ENDED,
  C_FIX_ERROR_LOGON_FAILED,
  C_FIX_ERROR_LOGOUT_FAILED,
  C_FIX_ERROR_SEND_MESSAGE_FAILED,
  C_FIX_ERROR_NULL_POINTER,
  C_FIX_ERROR_BAD_STRING,
  C_FIX_ERROR_SETTING_REQUIRED,
  C_FIX_ERROR_UNKNOWN,
} c_fix_error;

typedef struct BlockingFixApplicationClient BlockingFixApplicationClient;

typedef struct MessageBuilder MessageBuilder;

typedef struct SessionSettings SessionSettings;

typedef struct SessionSettingsBuilder SessionSettingsBuilder;

typedef struct BlockingFixApplicationClient *fix_app_client_t;

typedef struct SessionSettings *session_settings_t;

typedef MessageBuilder *message_builder_t;

typedef struct SessionSettingsBuilder *session_settings_builder_t;

/**
 * # Safety
 *
 * This function should be called with Utf-8 valid strings.
 */
fix_app_client_t fix_app_client_build(session_settings_t settings);

/**
 * # Safety
 *
 * The fix_app_client_t will be mutated using this function.
 */
enum c_fix_error fix_app_client_start(fix_app_client_t client);

/**
 * # Safety
 *
 * fix_app_client_t should not be NULL.
 */
enum c_fix_error fix_app_client_end(fix_app_client_t client);

/**
 * # Safety
 *
 * Only pass in a fix_app_client_t that came from fix_app_client_new function.
 */
void fix_app_client_free(fix_app_client_t client);

/**
 * # Safety
 *
 * Neither fix_app_client_t nor message_builder_t should be NULL when passed in. message_builder_t
 * will be NULL if the function retures OK.
 */
enum c_fix_error fix_app_client_send_message(fix_app_client_t client, message_builder_t builder);

/**
 * # Safety
 *
 * Only Utf-8 valid strings and valid FIX MsgTypes should be passed to this function.
 */
message_builder_t message_builder_new(const char *begin_string, char msg_type);

/**
 * # Safety
 *
 * The message_builder_t should not be NULL.
 */
enum c_fix_error message_builder_push_str(message_builder_t builder,
                                          tags tag_param,
                                          const char *value);

/**
 * # Safety
 *
 * The message_builder_t should not be NULL.
 */
enum c_fix_error message_builder_push_int(message_builder_t builder,
                                          tags tag_param,
                                          intptr_t value);

/**
 * # Safety
 *
 * The message_builder_t should not be NULL.
 */
enum c_fix_error message_builder_push_field(message_builder_t builder,
                                            tags tag_param,
                                            intptr_t value);

/**
 * # Safety
 *
 * The message_builder_t should not be NULL.
 */
enum c_fix_error message_builder_push_current_time(message_builder_t builder, tags tag_param);

/**
 * # Safety
 *
 * Any pointers passed in should only be from the message_builder_new() function.
 */
void message_builder_free(message_builder_t builder);

session_settings_builder_t session_settings_builder_new(void);

/**
 * # Safety
 *
 * This function should be called with Utf-8 valid strings.
 */
enum c_fix_error ssb_set_sender_comp_id(session_settings_builder_t builder,
                                        const char *sender_comp_id);

/**
 * # Safety
 *
 * This function should be called with Utf-8 valid strings.
 */
enum c_fix_error ssb_set_target_comp_id(session_settings_builder_t builder,
                                        const char *target_comp_id);

/**
 * # Safety
 *
 * This function should be called with Utf-8 valid strings.
 */
enum c_fix_error ssb_set_socket_addr(session_settings_builder_t builder, const char *addr);

/**
 * # Safety
 *
 * The pointers should not be NULL.
 */
enum c_fix_error ssb_set_begin_string(session_settings_builder_t builder, const char *begin_string);

/**
 * # Safety
 *
 * The pointers should not be NULL.
 */
enum c_fix_error ssb_set_epoch(session_settings_builder_t builder, const char *epoch);

/**
 * # Safety
 *
 * The pointers should not be NULL.
 */
enum c_fix_error ssb_set_store_path(session_settings_builder_t builder, const char *store_path);

/**
 * # Safety
 *
 * The pointers should not be NULL.
 */
enum c_fix_error ssb_set_log_dir(session_settings_builder_t builder, const char *log_dir);

/**
 * # Safety
 *
 * The pointers should not be NULL.
 */
enum c_fix_error ssb_set_heartbeat_timeout(session_settings_builder_t builder,
                                           unsigned long heartbeat_timeout);

/**
 * # Safety
 *
 * The pointers should not be NULL and start_time_str should be a UTC time in format `HH:MM:SS`.
 */
enum c_fix_error ssb_set_start_time(session_settings_builder_t builder, const char *start_time_str);

/**
 * # Safety
 *
 * The pointer should not be NULL.
 */
session_settings_t ssb_build(session_settings_builder_t builder);

/**
 * # Safety
 *
 * Any pointers passed in should only be from the session_settings_builder_new() function.
 */
void session_settings_builder_free(session_settings_builder_t builder);

#endif /* _FIX_H */
