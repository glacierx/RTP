pub const ERROR_NONE: TThostFtdcErrorIDType = 0;
pub const ERROR_INVALID_DATA_SYNC_STATUS: TThostFtdcErrorIDType = 1;
pub const ERROR_INCONSISTENT_INFORMATION: TThostFtdcErrorIDType = 2;
pub const ERROR_INVALID_LOGIN: TThostFtdcErrorIDType = 3;
pub const ERROR_USER_NOT_ACTIVE: TThostFtdcErrorIDType = 4;
pub const ERROR_DUPLICATE_LOGIN: TThostFtdcErrorIDType = 5;
pub const ERROR_NOT_LOGIN_YET: TThostFtdcErrorIDType = 6;
pub const ERROR_NOT_INITED: TThostFtdcErrorIDType = 7;
pub const ERROR_FRONT_NOT_ACTIVE: TThostFtdcErrorIDType = 8;
pub const ERROR_NO_PRIVILEGE: TThostFtdcErrorIDType = 9;
pub const ERROR_CHANGE_OTHER_PASSWORD: TThostFtdcErrorIDType = 10;
pub const ERROR_USER_NOT_FOUND: TThostFtdcErrorIDType = 11;
pub const ERROR_BROKER_NOT_FOUND: TThostFtdcErrorIDType = 12;
pub const ERROR_INVESTOR_NOT_FOUND: TThostFtdcErrorIDType = 13;
pub const ERROR_OLD_PASSWORD_MISMATCH: TThostFtdcErrorIDType = 14;
pub const ERROR_BAD_FIELD: TThostFtdcErrorIDType = 15;
pub const ERROR_INSTRUMENT_NOT_FOUND: TThostFtdcErrorIDType = 16;
pub const ERROR_INSTRUMENT_NOT_TRADING: TThostFtdcErrorIDType = 17;
pub const ERROR_NOT_EXCHANGE_PARTICIPANT: TThostFtdcErrorIDType = 18;
pub const ERROR_INVESTOR_NOT_ACTIVE: TThostFtdcErrorIDType = 19;
pub const ERROR_NOT_EXCHANGE_CLIENT: TThostFtdcErrorIDType = 20;
pub const ERROR_NO_VALID_TRADER_AVAILABLE: TThostFtdcErrorIDType = 21;
pub const ERROR_DUPLICATE_ORDER_REF: TThostFtdcErrorIDType = 22;
pub const ERROR_BAD_ORDER_ACTION_FIELD: TThostFtdcErrorIDType = 23;
pub const ERROR_DUPLICATE_ORDER_ACTION_REF: TThostFtdcErrorIDType = 24;
pub const ERROR_ORDER_NOT_FOUND: TThostFtdcErrorIDType = 25;
pub const ERROR_INSUITABLE_ORDER_STATUS: TThostFtdcErrorIDType = 26;
pub const ERROR_UNSUPPORTED_FUNCTION: TThostFtdcErrorIDType = 27;
pub const ERROR_NO_TRADING_RIGHT: TThostFtdcErrorIDType = 28;
pub const ERROR_CLOSE_ONLY: TThostFtdcErrorIDType = 29;
pub const ERROR_OVER_CLOSE_POSITION: TThostFtdcErrorIDType = 30;
pub const ERROR_INSUFFICIENT_MONEY: TThostFtdcErrorIDType = 31;
pub const ERROR_DUPLICATE_PK: TThostFtdcErrorIDType = 32;
pub const ERROR_CANNOT_FIND_PK: TThostFtdcErrorIDType = 33;
pub const ERROR_CAN_NOT_INACTIVE_BROKER: TThostFtdcErrorIDType = 34;
pub const ERROR_BROKER_SYNCHRONIZING: TThostFtdcErrorIDType = 35;
pub const ERROR_BROKER_SYNCHRONIZED: TThostFtdcErrorIDType = 36;
pub const ERROR_SHORT_SELL: TThostFtdcErrorIDType = 37;
pub const ERROR_INVALID_SETTLEMENT_REF: TThostFtdcErrorIDType = 38;
pub const ERROR_CFFEX_NETWORK_ERROR: TThostFtdcErrorIDType = 39;
pub const ERROR_CFFEX_OVER_REQUEST: TThostFtdcErrorIDType = 40;
pub const ERROR_CFFEX_OVER_REQUEST_PER_SECOND: TThostFtdcErrorIDType = 41;
pub const ERROR_SETTLEMENT_INFO_NOT_CONFIRMED: TThostFtdcErrorIDType = 42;
pub const ERROR_DEPOSIT_NOT_FOUND: TThostFtdcErrorIDType = 43;
pub const ERROR_EXCHANG_TRADING: TThostFtdcErrorIDType = 44;
pub const ERROR_PARKEDORDER_NOT_FOUND: TThostFtdcErrorIDType = 45;
pub const ERROR_PARKEDORDER_HASSENDED: TThostFtdcErrorIDType = 46;
pub const ERROR_PARKEDORDER_HASDELETE: TThostFtdcErrorIDType = 47;
pub const ERROR_INVALID_INVESTORIDORPASSWORD: TThostFtdcErrorIDType = 48;
pub const ERROR_INVALID_LOGIN_IPADDRESS: TThostFtdcErrorIDType = 49;
pub const ERROR_OVER_CLOSETODAY_POSITION: TThostFtdcErrorIDType = 50;
pub const ERROR_OVER_CLOSEYESTERDAY_POSITION: TThostFtdcErrorIDType = 51;
pub const ERROR_BROKER_NOT_ENOUGH_CONDORDER: TThostFtdcErrorIDType = 52;
pub const ERROR_INVESTOR_NOT_ENOUGH_CONDORDER: TThostFtdcErrorIDType = 53;
pub const ERROR_BROKER_NOT_SUPPORT_CONDORDER: TThostFtdcErrorIDType = 54;
pub const ERROR_RESEND_ORDER_BROKERINVESTOR_NOTMATCH: TThostFtdcErrorIDType = 55;
pub const ERROR_SYC_OTP_FAILED: TThostFtdcErrorIDType = 56;
pub const ERROR_OTP_MISMATCH: TThostFtdcErrorIDType = 57;
pub const ERROR_OTPPARAM_NOT_FOUND: TThostFtdcErrorIDType = 58;
pub const ERROR_UNSUPPORTED_OTPTYPE: TThostFtdcErrorIDType = 59;
pub const ERROR_SINGLEUSERSESSION_EXCEED_LIMIT: TThostFtdcErrorIDType = 60;
pub const ERROR_EXCHANGE_UNSUPPORTED_ARBITRAGE: TThostFtdcErrorIDType = 61;
pub const ERROR_NO_CONDITIONAL_ORDER_RIGHT: TThostFtdcErrorIDType = 62;
pub const ERROR_AUTH_FAILED: TThostFtdcErrorIDType = 63;
pub const ERROR_NOT_AUTHENT: TThostFtdcErrorIDType = 64;
pub const ERROR_SWAPORDER_UNSUPPORTED: TThostFtdcErrorIDType = 65;
pub const ERROR_OPTIONS_ONLY_SUPPORT_SPEC: TThostFtdcErrorIDType = 66;
pub const ERROR_DUPLICATE_EXECORDER_REF: TThostFtdcErrorIDType = 67;
pub const ERROR_RESEND_EXECORDER_BROKERINVESTOR_NOTMATCH: TThostFtdcErrorIDType = 68;
pub const ERROR_EXECORDER_NOTOPTIONS: TThostFtdcErrorIDType = 69;
pub const ERROR_OPTIONS_NOT_SUPPORT_EXEC: TThostFtdcErrorIDType = 70;
pub const ERROR_BAD_EXECORDER_ACTION_FIELD: TThostFtdcErrorIDType = 71;
pub const ERROR_DUPLICATE_EXECORDER_ACTION_REF: TThostFtdcErrorIDType = 72;
pub const ERROR_EXECORDER_NOT_FOUND: TThostFtdcErrorIDType = 73;
pub const ERROR_OVER_EXECUTE_POSITION: TThostFtdcErrorIDType = 74;
pub const ERROR_LOGIN_FORBIDDEN: TThostFtdcErrorIDType = 75;
pub const ERROR_INVALID_TRANSFER_AGENT: TThostFtdcErrorIDType = 76;
pub const ERROR_NO_FOUND_FUNCTION: TThostFtdcErrorIDType = 77;
pub const ERROR_SEND_EXCHANGEORDER_FAILED: TThostFtdcErrorIDType = 78;
pub const ERROR_SEND_EXCHANGEORDERACTION_FAILED: TThostFtdcErrorIDType = 79;
pub const ERROR_PRICETYPE_NOTSUPPORT_BYEXCHANGE: TThostFtdcErrorIDType = 80;
pub const ERROR_BAD_EXECUTE_TYPE: TThostFtdcErrorIDType = 81;
pub const ERROR_BAD_OPTION_INSTR: TThostFtdcErrorIDType = 82;
pub const ERROR_INSTR_NOTSUPPORT_FORQUOTE: TThostFtdcErrorIDType = 83;
pub const ERROR_RESEND_QUOTE_BROKERINVESTOR_NOTMATCH: TThostFtdcErrorIDType = 84;
pub const ERROR_INSTR_NOTSUPPORT_QUOTE: TThostFtdcErrorIDType = 85;
pub const ERROR_QUOTE_NOT_FOUND: TThostFtdcErrorIDType = 86;
pub const ERROR_OPTIONS_NOT_SUPPORT_ABANDON: TThostFtdcErrorIDType = 87;
pub const ERROR_COMBOPTIONS_SUPPORT_IOC_ONLY: TThostFtdcErrorIDType = 88;
pub const ERROR_OPEN_FILE_FAILED: TThostFtdcErrorIDType = 89;
pub const ERROR_NEED_RETRY: TThostFtdcErrorIDType = 90;
pub const ERROR_EXCHANGE_RTNERROR: TThostFtdcErrorIDType = 91;
pub const ERROR_QUOTE_DERIVEDORDER_ACTIONERROR: TThostFtdcErrorIDType = 92;
pub const ERROR_INSTRUMENTMAP_NOT_FOUND: TThostFtdcErrorIDType = 93;
pub const ERROR_CANCELLATION_OF_OTC_DERIVED_ORDER_NOT_ALLOWED: TThostFtdcErrorIDType = 94;
pub const ERROR_BAD_PRICE_VALUE: TThostFtdcErrorIDType = 95;
pub const ERROR_SPBMFUTPARAM_NOT_FOUND: TThostFtdcErrorIDType = 96;
pub const ERROR_SPBMOPTPARAM_NOT_FOUND: TThostFtdcErrorIDType = 97;
pub const ERROR_SPBMINTRAPARAM_NOT_FOUND: TThostFtdcErrorIDType = 98;
pub const ERROR_RULEINSTRPARAM_NOT_FOUND: TThostFtdcErrorIDType = 99;
pub const ERROR_RULEINTRAPARAM_NOT_FOUND: TThostFtdcErrorIDType = 100;
pub const ERROR_NO_TRADING_RIGHT_IN_SEPC_DR: TThostFtdcErrorIDType = 101;
pub const ERROR_NO_DR_NO: TThostFtdcErrorIDType = 102;
pub const ERROR_BATCHACTION_NOSUPPORT: TThostFtdcErrorIDType = 103;
pub const ERROR_POSI_LIMIT: TThostFtdcErrorIDType = 106;
pub const ERROR_OUT_OF_TIMEINTERVAL: TThostFtdcErrorIDType = 113;
pub const ERROR_OUT_OF_PRICEINTERVAL: TThostFtdcErrorIDType = 114;
pub const ERROR_ORDER_FREQ_LIMIT: TThostFtdcErrorIDType = 116;
pub const ERROR_WEAK_PASSWORD: TThostFtdcErrorIDType = 131;
pub const ERROR_EXEC_FORBIDDEN_TIME: TThostFtdcErrorIDType = 139;
pub const ERROR_FIRST_LOGIN: TThostFtdcErrorIDType = 140;
pub const ERROR_PWD_OUT_OF_DATE: TThostFtdcErrorIDType = 141;
pub const ERROR_PWD_MUST_DIFF: TThostFtdcErrorIDType = 142;
pub const ERROR_IP_FORBIDDEN: TThostFtdcErrorIDType = 143;
pub const ERROR_IP_BLACK: TThostFtdcErrorIDType = 144;
pub const ERROR_NO_AUTH_RIGHT_IN_SEPC_DR: TThostFtdcErrorIDType = 145;
pub const ERROR_INVESTOR_ID_IS_MISSING: TThostFtdcErrorIDType = 146;
pub const ERROR_EXCHANGE_ID_IS_MISSING: TThostFtdcErrorIDType = 147;
pub const ERROR_EXCHANGE_ID_IS_INVALID: TThostFtdcErrorIDType = 148;
pub const ERROR_ACCOUNT_ID_IS_MISSING: TThostFtdcErrorIDType = 149;
pub const ERROR_EXCHANGE_ID_IS_WRONG: TThostFtdcErrorIDType = 150;
pub const ERROR_DEL_COMB_ACTION_NO_REC: TThostFtdcErrorIDType = 151;
pub const ERROR_DEL_COMB_ACTION_TOO_FAST: TThostFtdcErrorIDType = 152;
pub const ERROR_COMB_ACTION_SHORT_MONEY: TThostFtdcErrorIDType = 153;
pub const ERROR_QK_BUSY: TThostFtdcErrorIDType = 154;
pub const ERROR_CFMMC_NO_CONNECTION: TThostFtdcErrorIDType = 155;
pub const ERROR_CLOSE_OPTION_NO_MONEY: TThostFtdcErrorIDType = 156;
pub const ERROR_CANCEL_UNKNOWN_ORDER_UNSUPPORTED: TThostFtdcErrorIDType = 157;
pub const ERROR_OVER_INFO_CNT_LIMIT: TThostFtdcErrorIDType = 158;
pub const ERROR_OVER_INVST_PARKED_ORDER_LIMIT: TThostFtdcErrorIDType = 159;
pub const ERROR_OVER_BROKER_PARKED_ORDER_LIMIT: TThostFtdcErrorIDType = 160;
pub const ERROR_PARKED_ORDER_WRONG_TYPE: TThostFtdcErrorIDType = 161;
pub const ERROR_SEND_INSTITUTION_CODE_ERROR: TThostFtdcErrorIDType = 1000;
pub const ERROR_NO_GET_PLATFORM_SN: TThostFtdcErrorIDType = 1001;
pub const ERROR_ILLEGAL_TRANSFER_BANK: TThostFtdcErrorIDType = 1002;
pub const ERROR_ALREADY_OPEN_ACCOUNT: TThostFtdcErrorIDType = 1003;
pub const ERROR_NOT_OPEN_ACCOUNT: TThostFtdcErrorIDType = 1004;
pub const ERROR_PROCESSING: TThostFtdcErrorIDType = 1005;
pub const ERROR_OVERTIME: TThostFtdcErrorIDType = 1006;
pub const ERROR_RECORD_NOT_FOUND: TThostFtdcErrorIDType = 1007;
pub const ERROR_NO_FOUND_REVERSAL_ORIGINAL_TRANSACTION: TThostFtdcErrorIDType = 1008;
pub const ERROR_CONNECT_HOST_FAILED: TThostFtdcErrorIDType = 1009;
pub const ERROR_SEND_FAILED: TThostFtdcErrorIDType = 1010;
pub const ERROR_LATE_RESPONSE: TThostFtdcErrorIDType = 1011;
pub const ERROR_REVERSAL_BANKID_NOT_MATCH: TThostFtdcErrorIDType = 1012;
pub const ERROR_REVERSAL_BANKACCOUNT_NOT_MATCH: TThostFtdcErrorIDType = 1013;
pub const ERROR_REVERSAL_BROKERID_NOT_MATCH: TThostFtdcErrorIDType = 1014;
pub const ERROR_REVERSAL_ACCOUNTID_NOT_MATCH: TThostFtdcErrorIDType = 1015;
pub const ERROR_REVERSAL_AMOUNT_NOT_MATCH: TThostFtdcErrorIDType = 1016;
pub const ERROR_DB_OPERATION_FAILED: TThostFtdcErrorIDType = 1017;
pub const ERROR_SEND_ASP_FAILURE: TThostFtdcErrorIDType = 1018;
pub const ERROR_NOT_SIGNIN: TThostFtdcErrorIDType = 1019;
pub const ERROR_ALREADY_SIGNIN: TThostFtdcErrorIDType = 1020;
pub const ERROR_AMOUNT_OR_TIMES_OVER: TThostFtdcErrorIDType = 1021;
pub const ERROR_NOT_IN_TRANSFER_TIME: TThostFtdcErrorIDType = 1022;
pub const ERROR_BANK_SERVER_ERROR: TThostFtdcErrorIDType = 1023;
pub const ERROR_BANK_SERIAL_IS_REPEALED: TThostFtdcErrorIDType = 1024;
pub const ERROR_BANK_SERIAL_NOT_EXIST: TThostFtdcErrorIDType = 1025;
pub const ERROR_NOT_ORGAN_MAP: TThostFtdcErrorIDType = 1026;
pub const ERROR_EXIST_TRANSFER: TThostFtdcErrorIDType = 1027;
pub const ERROR_BANK_FORBID_REVERSAL: TThostFtdcErrorIDType = 1028;
pub const ERROR_DUP_BANK_SERIAL: TThostFtdcErrorIDType = 1029;
pub const ERROR_FBT_SYSTEM_BUSY: TThostFtdcErrorIDType = 1030;
pub const ERROR_MACKEY_SYNCING: TThostFtdcErrorIDType = 1031;
pub const ERROR_ACCOUNTID_ALREADY_REGISTER: TThostFtdcErrorIDType = 1032;
pub const ERROR_BANKACCOUNT_ALREADY_REGISTER: TThostFtdcErrorIDType = 1033;
pub const ERROR_DUP_BANK_SERIAL_REDO_OK: TThostFtdcErrorIDType = 1034;
pub const ERROR_CURRENCYID_NOT_SUPPORTED: TThostFtdcErrorIDType = 1035;
pub const ERROR_INVALID_MAC: TThostFtdcErrorIDType = 1036;
pub const ERROR_NOT_SUPPORT_SECAGENT_BY_BANK: TThostFtdcErrorIDType = 1037;
pub const ERROR_PINKEY_SYNCING: TThostFtdcErrorIDType = 1038;
pub const ERROR_SECAGENT_QUERY_BY_CCB: TThostFtdcErrorIDType = 1039;
pub const ERROR_BANKACCOUNT_NOT_EMPTY: TThostFtdcErrorIDType = 1040;
pub const ERROR_INVALID_RESERVE_OPEN_ACCOUNT: TThostFtdcErrorIDType = 1041;
pub const ERROR_OPEN_ACCOUNT_NOT_DEFAULT_ACCOUNT: TThostFtdcErrorIDType = 1042;
pub const ERROR_BANK_SYSTEM_INTERNAL_ERROR: TThostFtdcErrorIDType = 1043;
pub const ERROR_OFFER_LOCALTIME_OFFSET_IS_TOO_LARGE: TThostFtdcErrorIDType = 1044;
pub const ERROR_FUTURESERIAL_HAS_BEEN_PROCESSED: TThostFtdcErrorIDType = 1045;
pub const ERROR_NO_VALID_BANKOFFER_AVAILABLE: TThostFtdcErrorIDType = 2000;
pub const ERROR_PASSWORD_MISMATCH: TThostFtdcErrorIDType = 2001;
pub const ERROR_DUPLATION_BANK_SERIAL: TThostFtdcErrorIDType = 2004;
pub const ERROR_DUPLATION_OFFER_SERIAL: TThostFtdcErrorIDType = 2005;
pub const ERROR_SERIAL_NOT_EXSIT: TThostFtdcErrorIDType = 2006;
pub const ERROR_SERIAL_IS_REPEALED: TThostFtdcErrorIDType = 2007;
pub const ERROR_SERIAL_MISMATCH: TThostFtdcErrorIDType = 2008;
pub const ERROR_IdentifiedCardNo_MISMATCH: TThostFtdcErrorIDType = 2009;
pub const ERROR_ACCOUNT_NOT_FUND: TThostFtdcErrorIDType = 2011;
pub const ERROR_ACCOUNT_NOT_ACTIVE: TThostFtdcErrorIDType = 2012;
pub const ERROR_NOT_ALLOW_REPEAL_BYMANUAL: TThostFtdcErrorIDType = 2013;
pub const ERROR_AMOUNT_OUTOFTHEWAY: TThostFtdcErrorIDType = 2014;
pub const ERROR_EXCHANGERATE_NOT_FOUND: TThostFtdcErrorIDType = 2015;
pub const ERROR_RESERVE_OPEN_ACCOUNT_NOT_FUND: TThostFtdcErrorIDType = 2016;
pub const ERROR_DUPLICATE_RESERVE_OPEN_ACCOUNT_NOT_FUND: TThostFtdcErrorIDType = 2017;
pub const ERROR_WAITING_OFFER_RSP: TThostFtdcErrorIDType = 999999;
pub const ERROR_PW_PASSWORD: TThostFtdcErrorIDType = 2050;
pub const ERROR_AL_AMOUNT_LIMITATION: TThostFtdcErrorIDType = 2051;
pub const ERROR_AC_AUTHORITY_CONTROL: TThostFtdcErrorIDType = 2052;
pub const ERROR_DC_DATA_CONTEXT: TThostFtdcErrorIDType = 2053;
pub const ERROR_CE_CONTENT_ERROR: TThostFtdcErrorIDType = 2054;
pub const ERROR_DO_DUPLICATE_OPERATION: TThostFtdcErrorIDType = 2055;
pub const ERROR_TM_TIME: TThostFtdcErrorIDType = 2056;
pub const ERROR_RC_RISK_CONTROL: TThostFtdcErrorIDType = 2057;
pub const ERROR_BL_BUSINESS_LOGIC: TThostFtdcErrorIDType = 2058;
pub const ERROR_NA_NA: TThostFtdcErrorIDType = 2059;
pub const ERROR_HW_HARDWARE: TThostFtdcErrorIDType = 2060;
pub const ERROR_IO_IO: TThostFtdcErrorIDType = 2062;
pub const ERROR_DB_DATABASE: TThostFtdcErrorIDType = 2063;
pub const ERROR_NC_NETWORK_COMMUNICATION: TThostFtdcErrorIDType = 2064;
pub const ERROR_SS_SECURITY_SERVICE: TThostFtdcErrorIDType = 2065;
pub const ERROR_CM_COMPONENTS: TThostFtdcErrorIDType = 2066;
pub const ERROR_FC_FLOW_CONTROL: TThostFtdcErrorIDType = 2067;
pub const ERROR_TL_TECHNICAL_LOGIC: TThostFtdcErrorIDType = 2069;
pub const ERROR_AT_ABSOLUTE_TECHNIQUE: TThostFtdcErrorIDType = 2070;
pub const ERROR_FBE_NO_GET_PLATFORM_SN: TThostFtdcErrorIDType = 3001;
pub const ERROR_FBE_ILLEGAL_TRANSFER_BANK: TThostFtdcErrorIDType = 3002;
pub const ERROR_FBE_PROCESSING: TThostFtdcErrorIDType = 3005;
pub const ERROR_FBE_OVERTIME: TThostFtdcErrorIDType = 3006;
pub const ERROR_FBE_RECORD_NOT_FOUND: TThostFtdcErrorIDType = 3007;
pub const ERROR_FBE_CONNECT_HOST_FAILED: TThostFtdcErrorIDType = 3009;
pub const ERROR_FBE_SEND_FAILED: TThostFtdcErrorIDType = 3010;
pub const ERROR_FBE_LATE_RESPONSE: TThostFtdcErrorIDType = 3011;
pub const ERROR_FBE_DB_OPERATION_FAILED: TThostFtdcErrorIDType = 3017;
pub const ERROR_FBE_NOT_SIGNIN: TThostFtdcErrorIDType = 3019;
pub const ERROR_FBE_ALREADY_SIGNIN: TThostFtdcErrorIDType = 3020;
pub const ERROR_FBE_AMOUNT_OR_TIMES_OVER: TThostFtdcErrorIDType = 3021;
pub const ERROR_FBE_NOT_IN_TRANSFER_TIME: TThostFtdcErrorIDType = 3022;
pub const ERROR_FBE_BANK_SERVER_ERROR: TThostFtdcErrorIDType = 3023;
pub const ERROR_FBE_NOT_ORGAN_MAP: TThostFtdcErrorIDType = 3026;
pub const ERROR_FBE_SYSTEM_BUSY: TThostFtdcErrorIDType = 3030;
pub const ERROR_FBE_CURRENCYID_NOT_SUPPORTED: TThostFtdcErrorIDType = 3035;
pub const ERROR_FBE_WRONG_BANK_ACCOUNT: TThostFtdcErrorIDType = 3036;
pub const ERROR_FBE_BANK_ACCOUNT_NO_FUNDS: TThostFtdcErrorIDType = 3037;
pub const ERROR_FBE_DUP_CERT_NO: TThostFtdcErrorIDType = 3038;
pub const ERROR_API_UNSUPPORTED_VERSION: TThostFtdcErrorIDType = 3039;
pub const ERROR_API_INVALID_KEY: TThostFtdcErrorIDType = 3040;
pub const ERROR_OPTION_SELF_CLOSE_NOT_OPTION: TThostFtdcErrorIDType = 3041;
pub const ERROR_OPTION_SELF_CLOSE_DUPLICATE_REF: TThostFtdcErrorIDType = 3042;
pub const ERROR_OPTION_SELF_CLOSE_BAD_FIELD: TThostFtdcErrorIDType = 3043;
pub const ERROR_OPTION_SELF_CLOSE_REC_NOT_FOUND: TThostFtdcErrorIDType = 3044;
pub const ERROR_OPTION_SELF_CLOSE_STATUS_ERR: TThostFtdcErrorIDType = 3045;
pub const ERROR_OPTION_SELF_CLOSE_DOUBLE_SET_ERR: TThostFtdcErrorIDType = 3046;
pub const ERROR_QUOTE_WRONG_HEDGE_TYPE: TThostFtdcErrorIDType = 3047;
pub const ERROR_API_FRONT_SHAKE_HAND_ERR: TThostFtdcErrorIDType = 4040;
pub const ERROR_DUPLICATE_SUBMIT: TThostFtdcErrorIDType = 4041;
pub const ERROR_AUTHIP_CHECK_ERR: TThostFtdcErrorIDType = 4042;
pub const ERROR_AUTHUSER_CHECK_ERR: TThostFtdcErrorIDType = 4043;
pub const ERROR_QUOTE_WRONG_REPALACE_SYSID: TThostFtdcErrorIDType = 4050;
pub const ERROR_AUTH_IP_FORBIDDEN: TThostFtdcErrorIDType = 4060;
pub const ERROR_MORTGAGE_NOT_BALANCE: TThostFtdcErrorIDType = 4061;
pub const ERROR_SMAPI_SSL_CONNECT_ERR: TThostFtdcErrorIDType = 4100;
pub const ERROR_SMAPI_WRONG_USERIDORNAME: TThostFtdcErrorIDType = 4101;
pub const ERROR_SMAPI_CERT_VERIFY_FAILED: TThostFtdcErrorIDType = 4102;
pub const ERROR_SMAPI_CERT_PROCESS_TIMEOUT: TThostFtdcErrorIDType = 4103;
pub const ERROR_SMAPI_LOGIN_ERROR: TThostFtdcErrorIDType = 4104;
pub const ERROR_SMAPI_SSL_CONNECT_TIMEOUT: TThostFtdcErrorIDType = 4105;
pub const ERROR_SMAPI_CERT_CONNECT_ERROR: TThostFtdcErrorIDType = 4106;
pub const ERROR_SMAPI_CERT_NOT_EXIST: TThostFtdcErrorIDType = 4107;
pub const ERROR_SMAPI_CERT_EXPIRED: TThostFtdcErrorIDType = 4108;
pub const ERROR_SMAPI_PIN_INCORRECT: TThostFtdcErrorIDType = 4109;
pub const ERROR_SMAPI_PIN_LOCKED: TThostFtdcErrorIDType = 4110;
pub const ERROR_SMAPI_LOAD_ERROR: TThostFtdcErrorIDType = 4111;
pub const ERROR_RCAMS_COMBPRODUCTINFO_NOT_FOUND: TThostFtdcErrorIDType = 5000;
pub const ERROR_RCAMS_SHORTOPTADJUSTPARAM_NOT_FOUND: TThostFtdcErrorIDType = 5001;
pub const ERROR_TK_BUSY: TThostFtdcErrorIDType = 5002;
pub const ERROR_OVER_SUB_INST_LIMIT: TThostFtdcErrorIDType = 6000;
pub fn error_id_to_chinese_description(error_id: TThostFtdcErrorIDType) -> &'static str {
    match error_id {
        ERROR_NONE => "CTP:姝ｇ‘",
        ERROR_INVALID_DATA_SYNC_STATUS => "CTP:涓嶅湪宸插悓姝ョ姸鎬�",
        ERROR_INCONSISTENT_INFORMATION => "CTP:浼氳瘽淇℃伅涓嶄竴鑷�",
        ERROR_INVALID_LOGIN => "CTP:涓嶅悎娉曠殑鐧诲綍",
        ERROR_USER_NOT_ACTIVE => "CTP:鐢ㄦ埛涓嶆椿璺�",
        ERROR_DUPLICATE_LOGIN => "CTP:閲嶅鐨勭櫥褰�",
        ERROR_NOT_LOGIN_YET => "CTP:杩樻病鏈夌櫥褰�",
        ERROR_NOT_INITED => "CTP:杩樻病鏈夊垵濮嬪寲",
        ERROR_FRONT_NOT_ACTIVE => "CTP:鍓嶇疆涓嶆椿璺�",
        ERROR_NO_PRIVILEGE => "CTP:鏃犳鏉冮檺",
        ERROR_CHANGE_OTHER_PASSWORD => "CTP:淇敼鍒汉鐨勫彛浠�",
        ERROR_USER_NOT_FOUND => "CTP:鎵句笉鍒拌鐢ㄦ埛",
        ERROR_BROKER_NOT_FOUND => "CTP:鎵句笉鍒拌缁忕邯鍏徃",
        ERROR_INVESTOR_NOT_FOUND => "CTP:鎵句笉鍒版姇璧勮€�",
        ERROR_OLD_PASSWORD_MISMATCH => "CTP:鍘熷彛浠や笉鍖归厤",
        ERROR_BAD_FIELD => "CTP:鎶ュ崟瀛楁鏈夎",
        ERROR_INSTRUMENT_NOT_FOUND => "CTP:鎵句笉鍒板悎绾�",
        ERROR_INSTRUMENT_NOT_TRADING => "CTP:鍚堢害涓嶈兘浜ゆ槗",
        ERROR_NOT_EXCHANGE_PARTICIPANT => "CTP:缁忕邯鍏徃涓嶆槸浜ゆ槗鎵€鐨勪細鍛�",
        ERROR_INVESTOR_NOT_ACTIVE => "CTP:鎶曡祫鑰呬笉娲昏穬",
        ERROR_NOT_EXCHANGE_CLIENT => "CTP:鎶曡祫鑰呮湭鍦ㄤ氦鏄撴墍寮€鎴�",
        ERROR_NO_VALID_TRADER_AVAILABLE => "CTP:璇ヤ氦鏄撳腑浣嶆湭杩炴帴鍒颁氦鏄撴墍",
        ERROR_DUPLICATE_ORDER_REF => "CTP:鎶ュ崟閿欒锛氫笉鍏佽閲嶅鎶ュ崟",
        ERROR_BAD_ORDER_ACTION_FIELD => "CTP:閿欒鐨勬姤鍗曟搷浣滃瓧娈�",
        ERROR_DUPLICATE_ORDER_ACTION_REF => "CTP:鎾ゅ崟宸叉姤閫侊紝涓嶅厑璁搁噸澶嶆挙鍗�",
        ERROR_ORDER_NOT_FOUND => "CTP:鎾ゅ崟鎵句笉鍒扮浉搴旀姤鍗�",
        ERROR_INSUITABLE_ORDER_STATUS => "CTP:鎶ュ崟宸插叏鎴愪氦鎴栧凡鎾ら攢锛屼笉鑳藉啀鎾�",
        ERROR_UNSUPPORTED_FUNCTION => "CTP:涓嶆敮鎸佺殑鍔熻兘",
        ERROR_NO_TRADING_RIGHT => "CTP:娌℃湁鎶ュ崟浜ゆ槗鏉冮檺",
        ERROR_CLOSE_ONLY => "CTP:鍙兘骞充粨",
        ERROR_OVER_CLOSE_POSITION => "CTP:骞充粨閲忚秴杩囨寔浠撻噺",
        ERROR_INSUFFICIENT_MONEY => "CTP:璧勯噾涓嶈冻",
        ERROR_DUPLICATE_PK => "CTP:涓婚敭閲嶅",
        ERROR_CANNOT_FIND_PK => "CTP:鎵句笉鍒颁富閿�",
        ERROR_CAN_NOT_INACTIVE_BROKER => "CTP:璁剧疆缁忕邯鍏徃涓嶆椿璺冪姸鎬佸け璐�",
        ERROR_BROKER_SYNCHRONIZING => "CTP:缁忕邯鍏徃姝ｅ湪鍚屾",
        ERROR_BROKER_SYNCHRONIZED => "CTP:缁忕邯鍏徃宸插悓姝�",
        ERROR_SHORT_SELL => "CTP:鐜拌揣浜ゆ槗涓嶈兘鍗栫┖",
        ERROR_INVALID_SETTLEMENT_REF => "CTP:涓嶅悎娉曠殑缁撶畻寮曠敤",
        ERROR_CFFEX_NETWORK_ERROR => "CTP:浜ゆ槗鎵€缃戠粶杩炴帴澶辫触",
        ERROR_CFFEX_OVER_REQUEST => "CTP:浜ゆ槗鎵€鏈鐞嗚姹傝秴杩囪鍙暟",
        ERROR_CFFEX_OVER_REQUEST_PER_SECOND => "CTP:浜ゆ槗鎵€姣忕鍙戦€佽姹傛暟瓒呰繃璁稿彲鏁�",
        ERROR_SETTLEMENT_INFO_NOT_CONFIRMED => "CTP:缁撶畻缁撴灉鏈‘璁�",
        ERROR_DEPOSIT_NOT_FOUND => "CTP:娌℃湁瀵瑰簲鐨勫叆閲戣褰�",
        ERROR_EXCHANG_TRADING => "CTP:浜ゆ槗鎵€宸茬粡杩涘叆杩炵画浜ゆ槗鐘舵€�",
        ERROR_PARKEDORDER_NOT_FOUND => "CTP:鎵句笉鍒伴鍩嬶紙鎾ゅ崟锛夊崟",
        ERROR_PARKEDORDER_HASSENDED => "CTP:棰勫煁锛堟挙鍗曪級鍗曞凡缁忓彂閫�",
        ERROR_PARKEDORDER_HASDELETE => "CTP:棰勫煁锛堟挙鍗曪級鍗曞凡缁忓垹闄�",
        ERROR_INVALID_INVESTORIDORPASSWORD => "CTP:鏃犳晥鐨勬姇璧勮€呮垨鑰呭瘑鐮�",
        ERROR_INVALID_LOGIN_IPADDRESS => "CTP:涓嶅悎娉曠殑鐧诲綍IP鍦板潃",
        ERROR_OVER_CLOSETODAY_POSITION => "CTP:骞充粖浠撲綅涓嶈冻",
        ERROR_OVER_CLOSEYESTERDAY_POSITION => "CTP:骞虫槰浠撲綅涓嶈冻",
        ERROR_BROKER_NOT_ENOUGH_CONDORDER => "CTP:缁忕邯鍏徃娌℃湁瓒冲鍙敤鐨勬潯浠跺崟鏁伴噺",
        ERROR_INVESTOR_NOT_ENOUGH_CONDORDER => "CTP:鎶曡祫鑰呮病鏈夎冻澶熷彲鐢ㄧ殑鏉′欢鍗曟暟閲�",
        ERROR_BROKER_NOT_SUPPORT_CONDORDER => "CTP:缁忕邯鍏徃涓嶆敮鎸佹潯浠跺崟",
        ERROR_RESEND_ORDER_BROKERINVESTOR_NOTMATCH => "CTP:閲嶅彂鏈煡鍗曠粡绾叕鍙�/鎶曡祫鑰呬笉鍖归厤",
        ERROR_SYC_OTP_FAILED => "CTP:鍚屾鍔ㄦ€佷护鐗屽け璐�",
        ERROR_OTP_MISMATCH => "CTP:鍔ㄦ€佷护鐗屾牎楠岄敊璇�",
        ERROR_OTPPARAM_NOT_FOUND => "CTP:鎵句笉鍒板姩鎬佷护鐗岄厤缃俊鎭�",
        ERROR_UNSUPPORTED_OTPTYPE => "CTP:涓嶆敮鎸佺殑鍔ㄦ€佷护鐗岀被鍨�",
        ERROR_SINGLEUSERSESSION_EXCEED_LIMIT => "CTP:鐢ㄦ埛鍦ㄧ嚎浼氳瘽瓒呭嚭涓婇檺",
        ERROR_EXCHANGE_UNSUPPORTED_ARBITRAGE => "CTP:璇ヤ氦鏄撴墍涓嶆敮鎸佸鍒�/鍋氬競鍟嗙被鍨嬫姤鍗�",
        ERROR_NO_CONDITIONAL_ORDER_RIGHT => "CTP:娌℃湁鏉′欢鍗曚氦鏄撴潈闄�",
        ERROR_AUTH_FAILED => "CTP:瀹㈡埛绔璇佸け璐�",
        ERROR_NOT_AUTHENT => "CTP:瀹㈡埛绔湭璁よ瘉",
        ERROR_SWAPORDER_UNSUPPORTED => "CTP:璇ュ悎绾︿笉鏀寔浜掓崲绫诲瀷鎶ュ崟",
        ERROR_OPTIONS_ONLY_SUPPORT_SPEC => "CTP:璇ユ湡鏉冨悎绾﹀彧鏀寔鎶曟満绫诲瀷鎶ュ崟",
        ERROR_DUPLICATE_EXECORDER_REF => "CTP:鎵ц瀹ｅ憡閿欒锛屼笉鍏佽閲嶅鎵ц",
        ERROR_RESEND_EXECORDER_BROKERINVESTOR_NOTMATCH => "CTP:閲嶅彂鏈煡鎵ц瀹ｅ憡缁忕邯鍏徃/鎶曡祫鑰呬笉鍖归厤",
        ERROR_EXECORDER_NOTOPTIONS => "CTP:鍙湁鏈熻揣鏈熸潈鍚堢害鍙墽琛�",
        ERROR_OPTIONS_NOT_SUPPORT_EXEC => "CTP:璇ユ湡鏉冨悎绾︿笉鏀寔鎵ц",
        ERROR_BAD_EXECORDER_ACTION_FIELD => "CTP:鎵ц瀹ｅ憡瀛楁鏈夎",
        ERROR_DUPLICATE_EXECORDER_ACTION_REF => "CTP:鎵ц瀹ｅ憡鎾ゅ崟宸叉姤閫侊紝涓嶅厑璁搁噸澶嶆挙鍗�",
        ERROR_EXECORDER_NOT_FOUND => "CTP:鎵ц瀹ｅ憡鎾ゅ崟鎵句笉鍒扮浉搴旀墽琛屽鍛�",
        ERROR_OVER_EXECUTE_POSITION => "CTP:鎵ц浠撲綅涓嶈冻",
        ERROR_LOGIN_FORBIDDEN => "CTP:杩炵画鐧诲綍澶辫触娆℃暟瓒呴檺锛岀櫥褰曡绂佹",
        ERROR_INVALID_TRANSFER_AGENT => "CTP:闈炴硶閾舵湡浠ｇ悊鍏崇郴",
        ERROR_NO_FOUND_FUNCTION => "CTP:鏃犳鍔熻兘",
        ERROR_SEND_EXCHANGEORDER_FAILED => "CTP:鍙戦€佹姤鍗曞け璐�",
        ERROR_SEND_EXCHANGEORDERACTION_FAILED => "CTP:鍙戦€佹姤鍗曟搷浣滃け璐�",
        ERROR_PRICETYPE_NOTSUPPORT_BYEXCHANGE => "CTP:浜ゆ槗鎵€涓嶆敮鎸佺殑浠锋牸绫诲瀷",
        ERROR_BAD_EXECUTE_TYPE => "CTP:閿欒鐨勬墽琛岀被鍨�",
        ERROR_BAD_OPTION_INSTR => "CTP:鏃犳晥鐨勭粍鍚堝悎绾�",
        ERROR_INSTR_NOTSUPPORT_FORQUOTE => "CTP:璇ュ悎绾︿笉鏀寔璇环",
        ERROR_RESEND_QUOTE_BROKERINVESTOR_NOTMATCH => "CTP:閲嶅彂鏈煡鎶ヤ环缁忕邯鍏徃/鎶曡祫鑰呬笉鍖归厤",
        ERROR_INSTR_NOTSUPPORT_QUOTE => "CTP:璇ュ悎绾︿笉鏀寔鎶ヤ环",
        ERROR_QUOTE_NOT_FOUND => "CTP:鎶ヤ环鎾ゅ崟鎵句笉鍒扮浉搴旀姤浠�",
        ERROR_OPTIONS_NOT_SUPPORT_ABANDON => "CTP:璇ユ湡鏉冨悎绾︿笉鏀寔鏀惧純鎵ц",
        ERROR_COMBOPTIONS_SUPPORT_IOC_ONLY => "CTP:璇ョ粍鍚堟湡鏉冨悎绾﹀彧鏀寔IOC",
        ERROR_OPEN_FILE_FAILED => "CTP:鎵撳紑鏂囦欢澶辫触",
        ERROR_NEED_RETRY => "CTP锛氭煡璇㈡湭灏辩华锛岃绋嶅悗閲嶈瘯",
        ERROR_EXCHANGE_RTNERROR => "CTP锛氫氦鏄撴墍杩斿洖鐨勯敊璇�",
        ERROR_QUOTE_DERIVEDORDER_ACTIONERROR => "CTP:鎶ヤ环琛嶇敓鍗曡绛夊緟浜ゆ槗鎵€杩斿洖鎵嶈兘鎾ゅ崟",
        ERROR_INSTRUMENTMAP_NOT_FOUND => "CTP:鎵句笉鍒扮粍鍚堝悎绾︽槧灏�",
        ERROR_CANCELLATION_OF_OTC_DERIVED_ORDER_NOT_ALLOWED => "CTP:涓嶅厑璁告挙閿€OTC琛嶇敓鎶ュ崟",
        ERROR_BAD_PRICE_VALUE => "CTP锛氫笉鏀寔鐨勪环鏍�",
        ERROR_SPBMFUTPARAM_NOT_FOUND => "CTP:鎵句笉鍒癝PBM鏈熻揣鍚堢害鍙傛暟",
        ERROR_SPBMOPTPARAM_NOT_FOUND => "CTP:鎵句笉鍒癝PBM鏈熸潈鍚堢害鍙傛暟",
        ERROR_SPBMINTRAPARAM_NOT_FOUND => "CTP:鎵句笉鍒癝PBM鍝佺鍐呭閿佷粨鎶樻墸鍙傛暟",
        ERROR_RULEINSTRPARAM_NOT_FOUND => "CTP:鎵句笉鍒癛ULE鍚堢害鍙傛暟",
        ERROR_RULEINTRAPARAM_NOT_FOUND => "CTP:鎵句笉鍒癛ULE鍝佺鍐呭閿佷粨鎶樻墸鍙傛暟",
        ERROR_NO_TRADING_RIGHT_IN_SEPC_DR => "CTP:鐢ㄦ埛鍦ㄦ湰绯荤粺娌℃湁鎶ュ崟鏉冮檺",
        ERROR_NO_DR_NO => "CTP:绯荤粺缂哄皯鐏惧鏍囩ず鍙�",
        ERROR_BATCHACTION_NOSUPPORT => "CTP:璇ヤ氦鏄撴墍涓嶆敮鎸佹壒閲忔挙鍗�",
        ERROR_POSI_LIMIT => "CTP:鎶曡祫鑰呴檺浠�",
        ERROR_OUT_OF_TIMEINTERVAL => "CTP:褰撳墠鏃堕棿绂佹璇环",
        ERROR_OUT_OF_PRICEINTERVAL => "CTP:褰撳墠浠峰樊绂佹璇环",
        ERROR_ORDER_FREQ_LIMIT => "CTP:涓嬪崟棰戠巼闄愬埗",
        ERROR_WEAK_PASSWORD => "CTP锛氭偍褰撳墠瀵嗙爜涓哄急瀵嗙爜锛岃淇敼鎴愬己瀵嗙爜鍚庨噸鏂扮櫥褰�",
        ERROR_EXEC_FORBIDDEN_TIME => "CTP:褰撳墠鏃堕棿绂佹琛屾潈",
        ERROR_FIRST_LOGIN => "CTP:棣栨鐧诲綍蹇呴』淇敼瀵嗙爜锛岃淇敼瀵嗙爜鍚庨噸鏂扮櫥褰�",
        ERROR_PWD_OUT_OF_DATE => "CTP:鎮ㄥ綋鍓嶅瘑鐮佸凡杩囨湡锛岃淇敼鍚庣櫥褰�",
        ERROR_PWD_MUST_DIFF => "CTP:淇敼瀵嗙爜澶辫触銆傛柊瀵嗙爜涓嶅厑璁镐笌鏃у瘑鐮佺浉鍚�",
        ERROR_IP_FORBIDDEN => "CTP:鎮ㄧ櫥褰曞け璐ユ鏁拌繃澶氾紝IP琚姝㈢櫥鍏TP",
        ERROR_IP_BLACK => "CTP:鎮ㄥ綋鍓岻P鍦ㄩ粦鍚嶅崟涓紝琚姝㈢櫥褰曞拰璁よ瘉",
        ERROR_NO_AUTH_RIGHT_IN_SEPC_DR => "CTP:缁堢鍦ㄦ湰绯荤粺娌℃湁璁よ瘉鏉冮檺",
        ERROR_INVESTOR_ID_IS_MISSING => "CTP:缂哄皯InvestorID瀛楁锛岃濉叆InvestorID",
        ERROR_EXCHANGE_ID_IS_MISSING => "CTP:缂哄皯ExchangeID瀛楁锛岃濉叆ExchangeID",
        ERROR_EXCHANGE_ID_IS_INVALID => "CTP:鏃犳晥鐨凟xchangeID瀛楁锛岃濉叆姝ｇ‘鐨凟xchangeID",
        ERROR_ACCOUNT_ID_IS_MISSING => "CTP:缂哄皯AccountID瀛楁锛岃濉叆AccountID",
        ERROR_EXCHANGE_ID_IS_WRONG => "CTP:浜ゆ槗鎵€浠ｇ爜閿欒",
        ERROR_DEL_COMB_ACTION_NO_REC => "CTP:鍒犻櫎鎷嗗垎缁勫悎鎸囦护锛氭病鏈夋壘鍒拌璁板綍",
        ERROR_DEL_COMB_ACTION_TOO_FAST => "CTP:鍒犻櫎鎷嗗垎缁勫悎鎸囦护锛氬師鎸囦护闇€瑕佺瓑寰�30s 鎵嶈兘鍒犻櫎",
        ERROR_COMB_ACTION_SHORT_MONEY => "CTP:鎷嗗垎缁勫悎閽变笉瓒�",
        ERROR_QK_BUSY => "CTP:鏌ヨ鏍稿績蹇� 璇风◢鍚庨噸璇�",
        ERROR_CFMMC_NO_CONNECTION => "CTP:鏈繛鎺ョ洃鎺т腑蹇�",
        ERROR_CLOSE_OPTION_NO_MONEY => "CTP:骞虫湡鏉冨澶村悗璧勯噾涓鸿礋锛堟敹鐩婂皬浜庡钩浠撴墜缁垂锛夛紝鍙彲鐢遍鎺т汉鍛樺己骞�",
        ERROR_CANCEL_UNKNOWN_ORDER_UNSUPPORTED => "CTP:璇ヤ氦鏄撴墍涓嶆敮鎸佹挙閿€鏈煡鍗�",
        ERROR_OVER_INFO_CNT_LIMIT => "CTP:瓒呰繃淇℃伅閲忛檺鍒�",
        ERROR_OVER_INVST_PARKED_ORDER_LIMIT => "CTP:瓒呰繃涓汉棰勫煁鍗曟渶澶ч噺闄愬埗",
        ERROR_OVER_BROKER_PARKED_ORDER_LIMIT => "CTP:瓒呰繃缁忕邯鍏徃棰勫煁鍗曟渶澶ч噺闄愬埗",
        ERROR_PARKED_ORDER_WRONG_TYPE => "CTP:棰勫煁鍗�:涓嶆敮鎸佺殑瑙﹀彂绫诲瀷",
        ERROR_SEND_INSTITUTION_CODE_ERROR => "CTP:閾舵湡杞处锛氬彂閫佹満鏋勪唬鐮侀敊璇�",
        ERROR_NO_GET_PLATFORM_SN => "CTP:閾舵湡杞处锛氬彇骞冲彴娴佹按鍙烽敊璇�",
        ERROR_ILLEGAL_TRANSFER_BANK => "CTP:閾舵湡杞处锛氫笉鍚堟硶鐨勮浆璐﹂摱琛�",
        ERROR_ALREADY_OPEN_ACCOUNT => "CTP:閾舵湡杞处锛氬凡缁忓紑鎴�",
        ERROR_NOT_OPEN_ACCOUNT => "CTP:閾舵湡杞处锛氭湭寮€鎴�",
        ERROR_PROCESSING => "CTP:閾舵湡杞处锛氬鐞嗕腑",
        ERROR_OVERTIME => "CTP:閾舵湡杞处锛氫氦鏄撹秴鏃�",
        ERROR_RECORD_NOT_FOUND => "CTP:閾舵湡杞处锛氭壘涓嶅埌璁板綍",
        ERROR_NO_FOUND_REVERSAL_ORIGINAL_TRANSACTION => "CTP:閾舵湡杞处锛氭壘涓嶅埌琚啿姝ｇ殑鍘熷浜ゆ槗",
        ERROR_CONNECT_HOST_FAILED => "CTP:閾舵湡杞处锛氳繛鎺ヤ富鏈哄け璐�",
        ERROR_SEND_FAILED => "CTP:閾舵湡杞处锛氬彂閫佸け璐�",
        ERROR_LATE_RESPONSE => "CTP:閾舵湡杞处锛氳繜鍒板簲绛�",
        ERROR_REVERSAL_BANKID_NOT_MATCH => "CTP:閾舵湡杞处锛氬啿姝ｄ氦鏄撻摱琛屼唬鐮侀敊璇�",
        ERROR_REVERSAL_BANKACCOUNT_NOT_MATCH => "CTP:閾舵湡杞处锛氬啿姝ｄ氦鏄撻摱琛岃处鎴烽敊璇�",
        ERROR_REVERSAL_BROKERID_NOT_MATCH => "CTP:閾舵湡杞处锛氬啿姝ｄ氦鏄撶粡绾叕鍙镐唬鐮侀敊璇�",
        ERROR_REVERSAL_ACCOUNTID_NOT_MATCH => "CTP:閾舵湡杞处锛氬啿姝ｄ氦鏄撹祫閲戣处鎴烽敊璇�",
        ERROR_REVERSAL_AMOUNT_NOT_MATCH => "CTP:閾舵湡杞处锛氬啿姝ｄ氦鏄撲氦鏄撻噾棰濋敊璇�",
        ERROR_DB_OPERATION_FAILED => "CTP:閾舵湡杞处锛氭暟鎹簱鎿嶄綔閿欒",
        ERROR_SEND_ASP_FAILURE => "CTP:閾舵湡杞处锛氬彂閫佸埌浜ゆ槗绯荤粺澶辫触",
        ERROR_NOT_SIGNIN => "CTP:閾舵湡杞处锛氭病鏈夌鍒�",
        ERROR_ALREADY_SIGNIN => "CTP:閾舵湡杞处锛氬凡缁忕鍒�",
        ERROR_AMOUNT_OR_TIMES_OVER => "CTP:閾舵湡杞处锛氶噾棰濇垨娆℃暟瓒呴檺",
        ERROR_NOT_IN_TRANSFER_TIME => "CTP:閾舵湡杞处锛氳繖涓€鏃堕棿娈典笉鑳借浆璐�",
        ERROR_BANK_SERVER_ERROR => "閾惰涓绘満閿�",
        ERROR_BANK_SERIAL_IS_REPEALED => "CTP:閾舵湡杞处锛氶摱琛屽凡缁忓啿姝�",
        ERROR_BANK_SERIAL_NOT_EXIST => "CTP:閾舵湡杞处锛氶摱琛屾祦姘翠笉瀛樺湪",
        ERROR_NOT_ORGAN_MAP => "CTP:閾舵湡杞处锛氭満鏋勬病鏈夌绾�",
        ERROR_EXIST_TRANSFER => "CTP:閾舵湡杞处锛氬瓨鍦ㄨ浆璐︼紝涓嶈兘閿€鎴�",
        ERROR_BANK_FORBID_REVERSAL => "CTP:閾舵湡杞处锛氶摱琛屼笉鏀寔鍐叉",
        ERROR_DUP_BANK_SERIAL => "CTP:閾舵湡杞处锛氶噸澶嶇殑閾惰娴佹按",
        ERROR_FBT_SYSTEM_BUSY => "CTP:閾舵湡杞处锛氳浆璐︾郴缁熷繖锛岀◢鍚庡啀璇�",
        ERROR_MACKEY_SYNCING => "CTP:閾舵湡杞处锛歁AC瀵嗛挜姝ｅ湪鍚屾",
        ERROR_ACCOUNTID_ALREADY_REGISTER => "CTP:閾舵湡杞处锛氳祫閲戣处鎴峰凡缁忕櫥璁�",
        ERROR_BANKACCOUNT_ALREADY_REGISTER => "CTP:閾舵湡杞处锛氶摱琛岃处鎴峰凡缁忕櫥璁�",
        ERROR_DUP_BANK_SERIAL_REDO_OK => "CTP:閾舵湡杞处锛氶噸澶嶇殑閾惰娴佹按,閲嶅彂鎴愬姛",
        ERROR_CURRENCYID_NOT_SUPPORTED => "CTP:閾舵湡杞处锛氳甯佺浠ｇ爜涓嶆敮鎸�",
        ERROR_INVALID_MAC => "CTP:閾舵湡杞处锛歁AC鍊奸獙璇佸け璐�",
        ERROR_NOT_SUPPORT_SECAGENT_BY_BANK => "CTP:閾舵湡杞处锛氫笉鏀寔閾惰绔彂璧风殑浜岀骇浠ｇ悊鍟嗚浆璐﹀拰鏌ヨ",
        ERROR_PINKEY_SYNCING => "CTP:閾舵湡杞处锛歅IN瀵嗛挜姝ｅ湪鍚屾",
        ERROR_SECAGENT_QUERY_BY_CCB => "CTP:閾舵湡杞处锛氬缓琛屽彂璧风殑浜岀骇浠ｇ悊鍟嗘煡璇�",
        ERROR_BANKACCOUNT_NOT_EMPTY => "CTP:閾舵湡杞处锛氶摱琛岃处鎴蜂笉鑳戒负绌�",
        ERROR_INVALID_RESERVE_OPEN_ACCOUNT => "CTP:閾舵湡杞处锛氳祫閲戣处鎴峰瓨鍦紝棰勭害寮€鎴峰け璐�",
        ERROR_OPEN_ACCOUNT_NOT_DEFAULT_ACCOUNT => "CTP:閾舵湡杞处锛氬紑鎴疯姹傜殑閾惰鍗″彿鍜岄鐣欑殑璐﹀彿涓嶅悓",
        ERROR_BANK_SYSTEM_INTERNAL_ERROR => "閾惰绯荤粺鍐呴儴閿欒",
        ERROR_OFFER_LOCALTIME_OFFSET_IS_TOO_LARGE => "閾舵湡杞处锛氶摱鏈熸姤鐩樻満鍣ㄦ椂闂村亸绉诲お澶�",
        ERROR_FUTURESERIAL_HAS_BEEN_PROCESSED => "閾舵湡杞处锛氳鏈熻揣娴佹按鍙峰凡缁忓鐞嗚繃",
        ERROR_NO_VALID_BANKOFFER_AVAILABLE => "CTP:璇ユ姤鐩樻湭杩炴帴鍒伴摱琛�",
        ERROR_PASSWORD_MISMATCH => "CTP:璧勯噾瀵嗙爜閿欒",
        ERROR_DUPLATION_BANK_SERIAL => "CTP:閾惰娴佹按鍙烽噸澶�",
        ERROR_DUPLATION_OFFER_SERIAL => "CTP:鎶ョ洏娴佹按鍙烽噸澶�",
        ERROR_SERIAL_NOT_EXSIT => "CTP:琚啿姝ｆ祦姘翠笉瀛樺湪(鍐叉浜ゆ槗)",
        ERROR_SERIAL_IS_REPEALED => "CTP:鍘熸祦姘村凡鍐叉(鍐叉浜ゆ槗)",
        ERROR_SERIAL_MISMATCH => "CTP:涓庡師娴佹按淇℃伅涓嶇(鍐叉浜ゆ槗)",
        ERROR_IdentifiedCardNo_MISMATCH => "CTP:璇佷欢鍙风爜鎴栫被鍨嬮敊璇�",
        ERROR_ACCOUNT_NOT_FUND => "CTP:璧勯噾璐︽埛涓嶅瓨鍦�",
        ERROR_ACCOUNT_NOT_ACTIVE => "CTP:璧勯噾璐︽埛宸茬粡閿€鎴�",
        ERROR_NOT_ALLOW_REPEAL_BYMANUAL => "CTP:璇ヤ氦鏄撲笉鑳芥墽琛屾墜宸ュ啿姝�",
        ERROR_AMOUNT_OUTOFTHEWAY => "CTP:杞笎閲戦閿欒",
        ERROR_EXCHANGERATE_NOT_FOUND => "CTP:鎵句笉鍒版眹鐜�",
        ERROR_RESERVE_OPEN_ACCOUNT_NOT_FUND => "CTP:鎵句笉鍒伴绾﹀紑鎴疯姹�",
        ERROR_DUPLICATE_RESERVE_OPEN_ACCOUNT_NOT_FUND => "CTP:閲嶅鐨勯绾﹀紑鎴疯姹�",
        ERROR_WAITING_OFFER_RSP => "CTP:绛夊緟閾舵湡鎶ョ洏澶勭悊缁撴灉",
        ERROR_PW_PASSWORD => "寤鸿閾舵湡锛氬瘑鐮佷笌璁よ瘉(涓氬姟閿欒)",
        ERROR_AL_AMOUNT_LIMITATION => "寤鸿閾舵湡锛氭暟閲忎笌闄愰(涓氬姟閿欒)",
        ERROR_AC_AUTHORITY_CONTROL => "寤鸿閾舵湡锛氭潈闄愭帶鍒�(涓氬姟閿欒)",
        ERROR_DC_DATA_CONTEXT => "寤鸿閾舵湡锛氫俊鎭互缂�(涓氬姟閿欒)鎴栬€呮暟鎹唴瀹圭浉鍏�(鎶€鏈敊璇�)",
        ERROR_CE_CONTENT_ERROR => "寤鸿閾舵湡锛氬唴瀹归潪娉�(涓氬姟閿欒)",
        ERROR_DO_DUPLICATE_OPERATION => "寤鸿閾舵湡锛氶噸澶嶄氦鏄�(涓氬姟閿欒)",
        ERROR_TM_TIME => "寤鸿閾舵湡锛� 鏃堕棿涓庢湡闄�(涓氬姟閿欒)",
        ERROR_RC_RISK_CONTROL => "寤鸿閾舵湡锛氶闄╂帶鍒�(涓氬姟閿欒)",
        ERROR_BL_BUSINESS_LOGIC => "寤鸿閾舵湡锛氫笟鍔￠€昏緫(涓氬姟閿欒)",
        ERROR_NA_NA => "寤鸿閾舵湡锛� 涓嶇‘瀹氫氦鏄撶粨鏋�(鎶€鏈敊璇�)",
        ERROR_HW_HARDWARE => "寤鸿閾舵湡锛� 纭欢閿欒(鎶€鏈敊璇�)",
        ERROR_IO_IO => "寤鸿閾舵湡锛� 璇诲啓鐩稿叧(鎶€鏈敊璇�)",
        ERROR_DB_DATABASE => "寤鸿閾舵湡锛� 鏁版嵁搴撶浉鍏�(鎶€鏈敊璇�)",
        ERROR_NC_NETWORK_COMMUNICATION => "寤鸿閾舵湡锛氱綉缁滈€氳(鎶€鏈敊璇�)",
        ERROR_SS_SECURITY_SERVICE => "寤鸿閾舵湡锛氬畨鍏ㄦ湇鍔�(鎶€鏈敊璇�)",
        ERROR_CM_COMPONENTS => "寤鸿閾舵湡锛� 缁勪欢妯″潡(鎶€鏈敊璇�)",
        ERROR_FC_FLOW_CONTROL => "寤鸿閾舵湡锛氭祦閲忔帶鍒�(鎶€鏈敊璇�)",
        ERROR_TL_TECHNICAL_LOGIC => "寤鸿閾舵湡锛氭妧鏈€昏緫(鎶€鏈敊璇�)",
        ERROR_AT_ABSOLUTE_TECHNIQUE => "寤鸿閾舵湡锛氱函鎶€鏈€ч敊璇�(鎶€鏈敊璇�)",
        ERROR_FBE_NO_GET_PLATFORM_SN => "CTP:閾舵湡鎹㈡眹锛氬彇骞冲彴娴佹按鍙烽敊璇�",
        ERROR_FBE_ILLEGAL_TRANSFER_BANK => "CTP:閾舵湡鎹㈡眹锛氫笉鍚堟硶鐨勮浆璐﹂摱琛�",
        ERROR_FBE_PROCESSING => "CTP:閾舵湡鎹㈡眹锛氬鐞嗕腑",
        ERROR_FBE_OVERTIME => "CTP:閾舵湡鎹㈡眹锛氫氦鏄撹秴鏃�",
        ERROR_FBE_RECORD_NOT_FOUND => "CTP:閾舵湡鎹㈡眹锛氭壘涓嶅埌璁板綍",
        ERROR_FBE_CONNECT_HOST_FAILED => "CTP:閾舵湡鎹㈡眹锛氳繛鎺ヤ富鏈哄け璐�",
        ERROR_FBE_SEND_FAILED => "CTP:閾舵湡鎹㈡眹锛氬彂閫佸け璐�",
        ERROR_FBE_LATE_RESPONSE => "CTP:閾舵湡鎹㈡眹锛氳繜鍒板簲绛�",
        ERROR_FBE_DB_OPERATION_FAILED => "CTP:閾舵湡鎹㈡眹锛氭暟鎹簱鎿嶄綔閿欒",
        ERROR_FBE_NOT_SIGNIN => "CTP:閾舵湡鎹㈡眹锛氭病鏈夌鍒�",
        ERROR_FBE_ALREADY_SIGNIN => "CTP:閾舵湡鎹㈡眹锛氬凡缁忕鍒�",
        ERROR_FBE_AMOUNT_OR_TIMES_OVER => "CTP:閾舵湡鎹㈡眹锛氶噾棰濇垨娆℃暟瓒呴檺",
        ERROR_FBE_NOT_IN_TRANSFER_TIME => "CTP:閾舵湡鎹㈡眹锛氳繖涓€鏃堕棿娈典笉鑳芥崲姹�",
        ERROR_FBE_BANK_SERVER_ERROR => "CTP:閾舵湡鎹㈡眹锛氶摱琛屼富鏈洪敊",
        ERROR_FBE_NOT_ORGAN_MAP => "CTP:閾舵湡鎹㈡眹锛氭満鏋勬病鏈夌绾�",
        ERROR_FBE_SYSTEM_BUSY => "CTP:閾舵湡鎹㈡眹锛氭崲姹囩郴缁熷繖锛岀◢鍚庡啀璇�",
        ERROR_FBE_CURRENCYID_NOT_SUPPORTED => "CTP:閾舵湡鎹㈡眹锛氳甯佺浠ｇ爜涓嶆敮鎸�",
        ERROR_FBE_WRONG_BANK_ACCOUNT => "CTP:閾舵湡鎹㈡眹锛氶摱琛屽笎鍙蜂笉姝ｇ‘",
        ERROR_FBE_BANK_ACCOUNT_NO_FUNDS => "CTP:閾舵湡鎹㈡眹锛氶摱琛屽笎鎴蜂綑棰濅笉瓒�",
        ERROR_FBE_DUP_CERT_NO => "CTP:閾舵湡鎹㈡眹锛氬嚟璇佸彿閲嶅",
        ERROR_API_UNSUPPORTED_VERSION => "CTP: 涓嶆敮鎸佽API鐗堟湰",
        ERROR_API_INVALID_KEY => "CTP: 鏃犳晥鐨凙PI KEY",
        ERROR_OPTION_SELF_CLOSE_NOT_OPTION => "CTP:鏈熸潈瀵瑰啿,灞ョ害瀵瑰啿:闈炴湡鏉冨悎绾�",
        ERROR_OPTION_SELF_CLOSE_DUPLICATE_REF => "CTP:鏈熸潈瀵瑰啿,灞ョ害瀵瑰啿:璇锋眰寮曠敤涓嶅悎娉�",
        ERROR_OPTION_SELF_CLOSE_BAD_FIELD => "CTP:鏈熸潈瀵瑰啿,灞ョ害瀵瑰啿:闈炴硶瀛楁 ",
        ERROR_OPTION_SELF_CLOSE_REC_NOT_FOUND => "CTP:鏈熸潈瀵瑰啿,灞ョ害瀵瑰啿:鎾ら攢鏈壘鍒拌褰�",
        ERROR_OPTION_SELF_CLOSE_STATUS_ERR => "CTP:鏈熸潈瀵瑰啿,灞ョ害瀵瑰啿:瀵瑰啿鐘舵€佷笉瀵癸紝涓嶈兘鎾ら攢",
        ERROR_OPTION_SELF_CLOSE_DOUBLE_SET_ERR => "CTP:鏈熸潈瀵瑰啿,灞ョ害瀵瑰啿:涓嶈兘閲嶅璁剧疆锛屽彧鑳藉厛鎾ら攢鍐嶈缃�",
        ERROR_QUOTE_WRONG_HEDGE_TYPE => "CTP:鎶ヤ环涓嶆敮鎸佽鎶曟満濂椾繚绫诲瀷",
        ERROR_API_FRONT_SHAKE_HAND_ERR => "CTP:API Front shake hand err",
        ERROR_DUPLICATE_SUBMIT => "CTP:DUPLICATE_SUBMIT",
        ERROR_AUTHIP_CHECK_ERR => "CTP:IP鎺堟潈楠岃瘉澶辫触",
        ERROR_AUTHUSER_CHECK_ERR => "CTP:鐢ㄦ埛涓庡鎴风鎺堟潈楠岃瘉澶辫触",
        ERROR_QUOTE_WRONG_REPALACE_SYSID => "CTP:鎶ヤ环鎸囧畾鐨勯《鍗曠紪鍙蜂笉鍚堟硶锛堜腑閲戞墍锛�",
        ERROR_AUTH_IP_FORBIDDEN => "CTP:鎮ㄨ璇佸け璐ユ鏁拌繃澶氾紝IP杩涘叆璁よ瘉绂佹鍒楄〃",
        ERROR_MORTGAGE_NOT_BALANCE => "CTP:鏈弧瓒宠川鎶奸厤姣旇姹�",
        ERROR_SMAPI_SSL_CONNECT_ERR => "CTP:SSL Connect Error.",
        ERROR_SMAPI_WRONG_USERIDORNAME => "CTP:Wrong User ID or Name.",
        ERROR_SMAPI_CERT_VERIFY_FAILED => "CTP:Cert Verify Failed.",
        ERROR_SMAPI_CERT_PROCESS_TIMEOUT => "CTP:SM Process Timeout.",
        ERROR_SMAPI_LOGIN_ERROR => "CTP:Login Error.",
        ERROR_SMAPI_SSL_CONNECT_TIMEOUT => "CTP:SSL Connect Timeout.",
        ERROR_SMAPI_CERT_CONNECT_ERROR => "CTP:Cert Connect Error.",
        ERROR_SMAPI_CERT_NOT_EXIST => "CTP:Cert Is Not Existed.",
        ERROR_SMAPI_CERT_EXPIRED => "CTP:Cert Is Expired.",
        ERROR_SMAPI_PIN_INCORRECT => "CTP:PIN Is Not Corrected.",
        ERROR_SMAPI_PIN_LOCKED => "CTP:PIN Is Locked.",
        ERROR_SMAPI_LOAD_ERROR => "CTP:SMApi Load Error.",
        ERROR_RCAMS_COMBPRODUCTINFO_NOT_FOUND => "CTP:鎵句笉鍒癛CAMS浜у搧缁勫悎淇℃伅",
        ERROR_RCAMS_SHORTOPTADJUSTPARAM_NOT_FOUND => "CTP:鎵句笉鍒癛CAMS绌哄ご鏈熸潈椋庨櫓璋冩暣鍙傛暟",
        ERROR_TK_BUSY => "CTP:绯荤粺蹇�",
        ERROR_OVER_SUB_INST_LIMIT => "CTP:sub too many insts",
        _ => "unknown error",
    }
}
