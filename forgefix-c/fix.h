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

fix_app_client_t fix_app_client_build(session_settings_t settings);

enum c_fix_error fix_app_client_start(fix_app_client_t client);

enum c_fix_error fix_app_client_end(fix_app_client_t client);

void fix_app_client_free(fix_app_client_t client);

enum c_fix_error fix_app_client_send_message(fix_app_client_t client, message_builder_t builder);

message_builder_t message_builder_new(const char *begin_string, char msg_type);

enum c_fix_error message_builder_push_str(message_builder_t builder,
                                          tags tag_param,
                                          const char *value);

enum c_fix_error message_builder_push_int(message_builder_t builder,
                                          tags tag_param,
                                          intptr_t value);

enum c_fix_error message_builder_push_field(message_builder_t builder,
                                            tags tag_param,
                                            intptr_t value);

enum c_fix_error message_builder_push_current_time(message_builder_t builder, tags tag_param);

void message_builder_free(message_builder_t builder);

session_settings_builder_t session_settings_builder_new(void);

enum c_fix_error ssb_set_sender_comp_id(session_settings_builder_t builder,
                                        const char *sender_comp_id);

enum c_fix_error ssb_set_target_comp_id(session_settings_builder_t builder,
                                        const char *target_comp_id);

enum c_fix_error ssb_set_socket_addr(session_settings_builder_t builder, const char *addr);

enum c_fix_error ssb_set_begin_string(session_settings_builder_t builder, const char *begin_string);

enum c_fix_error ssb_set_epoch(session_settings_builder_t builder, const char *epoch);

enum c_fix_error ssb_set_store_path(session_settings_builder_t builder, const char *store_path);

enum c_fix_error ssb_set_log_dir(session_settings_builder_t builder, const char *log_dir);

enum c_fix_error ssb_set_heartbeat_timeout(session_settings_builder_t builder,
                                           unsigned long heartbeat_timeout);

enum c_fix_error ssb_set_start_time(session_settings_builder_t builder, const char *start_time_str);

session_settings_t ssb_build(session_settings_builder_t builder);

void session_settings_builder_free(session_settings_builder_t builder);

#endif /* _FIX_H */
