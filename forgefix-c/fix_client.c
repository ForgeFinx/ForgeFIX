#include <stdio.h>
#include <stdint.h>
#include <unistd.h>
#include "fix.h"
#include "fix_fields.h"

const char ID_SOURCE  = 'A';

c_fix_error send_order(
    fix_app_client_t fix_app_client, 
    const char* sguid, 
    uint32_t qty,
    const char* symbol,
    const char* price,
    uint32_t is_buy, 
    const char* exchange, 
    const char* account); 


int c_main(const char *log, const char *store) {

    c_fix_error err; 

    session_settings_builder_t builder = session_settings_builder_new();
    ssb_set_sender_comp_id(builder, "TW");
    ssb_set_target_comp_id(builder, "ISLD");
    ssb_set_socket_addr(builder, "127.0.0.1:9000");
    ssb_set_begin_string(builder, "FIX.4.2");
    ssb_set_epoch(builder, "999");
    ssb_set_store_path(builder, store);
    ssb_set_log_dir(builder, log); 
    ssb_set_heartbeat_timeout(builder, 30); 
    ssb_set_start_time(builder, "23:59:59"); 
    ssb_set_reset_flag_on_initial_logon(builder, 1); 

    session_settings_t settings = ssb_build(builder); 
    if (settings == NULL) {
        printf("session_settings_builder failed to build\n");
        return C_FIX_ERROR_UNKNOWN;
    }

    fix_app_client_t app = fix_app_client_build(settings); 
    if (app == NULL) {
        printf("fix_app_client failed to build\n");
        return C_FIX_ERROR_UNKNOWN; 
    }

    err = fix_app_client_start(app);
    if (err) {
        printf("fix_app_client failed to start\n");
        printf("%d\n", err);
        return err;
    }

    err = send_order(app, "ID1", 1, "AAPL  230803P00100000", "2.31", 1, "ELMD", "ABCD1234");
    if (err) {
        return err;
    }

    sleep(1); 

    err = send_order(app, "ID2", 1, "AAPL  230803P00100000", "2.31", 1, "ELMD", "ABCD1234");
    if (err) {
        return err;
    }

    sleep(1); 

    err = fix_app_client_end(app);
    if (err) {
        printf("fix_app_client failed to end\n");
        return err;
    }
    fix_app_client_free(app);

    return 0; 
}

c_fix_error send_order(
    fix_app_client_t fix_app_client, 
    const char* sguid, 
    uint32_t qty,
    const char* symbol,
    const char* price,
    uint32_t is_buy, 
    const char* exchange, 
    const char* account) 
{
    msg_type mt = MSG_TYPE_ORDER_SINGLE;
    
    message_builder_t mb = message_builder_new("FIX.4.2", mt); 
    if (fix_app_client == NULL) {
        return C_FIX_ERROR_NULL_POINTER;
    }

    side s; 
    if (is_buy) {
        s = SIDE_BUY;
    } else {
        s = SIDE_SELL;
    }

    message_builder_push_str(mb, TAGS_ACCOUNT, account);
    message_builder_push_str(mb, TAGS_CL_ORD_ID, sguid); 
    message_builder_push_field(mb, TAGS_ID_SOURCE, ID_SOURCE);
    message_builder_push_int(mb, TAGS_ORDER_QTY, qty); 
    message_builder_push_field(mb, TAGS_ORD_TYPE, ORD_TYPE_LIMIT); 
    message_builder_push_str(mb, TAGS_PRICE, price); 
    message_builder_push_str(mb, TAGS_SECURITY_ID, symbol);
    message_builder_push_field(mb, TAGS_SIDE, s); 
    message_builder_push_field(mb, TAGS_TIME_IN_FORCE, TIME_IN_FORCE_IMMEDIATE_OR_CANCEL);
    message_builder_push_current_time(mb, TAGS_TRANSACT_TIME); 
    message_builder_push_field(mb, TAGS_OPEN_CLOSE, OPEN_CLOSE_OPEN);
    message_builder_push_str(mb, TAGS_EX_DESTINATION, exchange); 
    if (mb == NULL) {
        return C_FIX_ERROR_NULL_POINTER; 
    }

    c_fix_error err;
    if ((err = fix_app_client_send_message(fix_app_client, mb)) != C_FIX_ERROR_OK) {
        printf("send_order: fix_app_client failed to send order\n");
        return err;
    }

    return C_FIX_ERROR_OK;    
}
