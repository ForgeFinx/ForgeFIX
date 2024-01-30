#![allow(non_camel_case_types, dead_code)]
#![allow(clippy::upper_case_acronyms)]

use crate::fix::decode::DecodeError;

pub fn is_session_message(msg_type: char) -> bool {
    matches!(msg_type, '0' | '1' | '2' | '3' | '4' | '5' | 'A')
}

#[repr(C)]
#[derive(Debug)]
pub enum Tags {
    Account = 1,
    AdvId = 2,
    AdvRefID = 3,
    AdvSide = 4,
    AdvTransType = 5,
    AvgPx = 6,
    BeginSeqNo = 7,
    BeginString = 8,
    BodyLength = 9,
    CheckSum = 10,
    ClOrdID = 11,
    Commission = 12,
    CommType = 13,
    CumQty = 14,
    Currency = 15,
    EndSeqNo = 16,
    ExecID = 17,
    ExecInst = 18,
    ExecRefID = 19,
    ExecTransType = 20,
    HandlInst = 21,
    IDSource = 22,
    IOIid = 23,
    IOIOthSvc = 24,
    IOIQltyInd = 25,
    IOIRefID = 26,
    IOIShares = 27,
    IOITransType = 28,
    LastCapacity = 29,
    LastMkt = 30,
    LastPx = 31,
    LastShares = 32,
    LinesOfText = 33,
    MsgSeqNum = 34,
    MsgType = 35,
    NewSeqNo = 36,
    OrderID = 37,
    OrderQty = 38,
    OrdStatus = 39,
    OrdType = 40,
    OrigClOrdID = 41,
    OrigTime = 42,
    PossDupFlag = 43,
    Price = 44,
    RefSeqNum = 45,
    RelatdSym = 46,
    Rule80A = 47,
    SecurityID = 48,
    SenderCompID = 49,
    SenderSubID = 50,
    SendingDate = 51,
    SendingTime = 52,
    Shares = 53,
    Side = 54,
    Symbol = 55,
    TargetCompID = 56,
    TargetSubID = 57,
    Text = 58,
    TimeInForce = 59,
    TransactTime = 60,
    Urgency = 61,
    ValidUntilTime = 62,
    SettlmntTyp = 63,
    FutSettDate = 64,
    SymbolSfx = 65,
    ListID = 66,
    ListSeqNo = 67,
    TotNoOrders = 68,
    ListExecInst = 69,
    AllocID = 70,
    AllocTransType = 71,
    RefAllocID = 72,
    NoOrders = 73,
    AvgPrxPrecision = 74,
    TradeDate = 75,
    ExecBroker = 76,
    OpenClose = 77,
    NoAllocs = 78,
    AllocAccount = 79,
    AllocShares = 80,
    ProcessCode = 81,
    NoRpts = 82,
    RptSeq = 83,
    CxlQty = 84,
    NoDlvyInst = 85,
    DlvyInst = 86,
    AllocStatus = 87,
    AllocRejCode = 88,
    Signature = 89,
    SecureDataLen = 90,
    SecureData = 91,
    BrokerOfCredit = 92,
    SignatureLength = 93,
    EmailType = 94,
    RawDataLength = 95,
    RawData = 96,
    PossResend = 97,
    EncryptMethod = 98,
    StopPx = 99,
    ExDestination = 100,
    CxlRejReason = 102,
    OrdRejReason = 103,
    IOIQualifier = 104,
    WaveNo = 105,
    Issuer = 106,
    SecurityDesc = 107,
    HeartBtInt = 108,
    ClientID = 109,
    MinQty = 110,
    MaxFloor = 111,
    TestReqID = 112,
    ReportToExch = 113,
    LocateReqd = 114,
    OnBehalfOfCompID = 115,
    OnBehalfOfSubID = 116,
    QuoteID = 117,
    NetMoney = 118,
    SettlCurrAmt = 119,
    SettlCurrency = 120,
    ForexReq = 121,
    OrigSendingTime = 122,
    GapFillFlag = 123,
    NoExecs = 124,
    CxlType = 125,
    ExpireTime = 126,
    DKReason = 127,
    DeliverToCompID = 128,
    DeliverToSubID = 129,
    IOINaturalFlag = 130,
    QuoteReqID = 131,
    BidPx = 132,
    OfferPx = 133,
    BidSize = 134,
    OfferSize = 135,
    NoMiscFees = 136,
    MiscFeeAmt = 137,
    MiscFeeCurr = 138,
    MiscFeeType = 139,
    PrevClosePx = 140,
    ResetSeqNumFlag = 141,
    SenderLocationID = 142,
    TargetLocationID = 143,
    OnBehalfOfLocationID = 144,
    DeliverToLocationID = 145,
    NoRelatedSym = 146,
    Subject = 147,
    Headline = 148,
    URLLink = 149,
    ExecType = 150,
    LeavesQty = 151,
    CashOrderQty = 152,
    AllocAvgPx = 153,
    AllocNetMoney = 154,
    SettlCurrFxRate = 155,
    SettlCurrFxRateCalc = 156,
    NumDaysInterest = 157,
    AccruedInterestRate = 158,
    AccruedInterestAmt = 159,
    SettlInstMode = 160,
    AllocText = 161,
    SettlInstID = 162,
    SettlInstTransType = 163,
    EmailThreadID = 164,
    SettlInstSource = 165,
    SettlLocation = 166,
    SecurityType = 167,
    EffectiveTime = 168,
    StandInstDbType = 169,
    StandInstDbName = 170,
    StandInstDbID = 171,
    SettlDeliveryType = 172,
    SettlDepositoryCode = 173,
    SettlBrkrCode = 174,
    SettlInstCode = 175,
    SecuritySettlAgentName = 176,
    SecuritySettlAgentCode = 177,
    SecuritySettlAgentAcctNum = 178,
    SecuritySettlAgentAcctName = 179,
    SecuritySettlAgentContactName = 180,
    SecuritySettlAgentContactPhone = 181,
    CashSettlAgentName = 182,
    CashSettlAgentCode = 183,
    CashSettlAgentAcctNum = 184,
    CashSettlAgentAcctName = 185,
    CashSettlAgentContactName = 186,
    CashSettlAgentContactPhone = 187,
    BidSpotRate = 188,
    BidForwardPoints = 189,
    OfferSpotRate = 190,
    OfferForwardPoints = 191,
    OrderQty2 = 192,
    FutSettDate2 = 193,
    LastSpotRate = 194,
    LastForwardPoints = 195,
    AllocLinkID = 196,
    AllocLinkType = 197,
    SecondaryOrderID = 198,
    NoIOIQualifiers = 199,
    MaturityMonthYear = 200,
    PutOrCall = 201,
    StrikePrice = 202,
    CoveredOrUncovered = 203,
    CustomerOrFirm = 204,
    MaturityDay = 205,
    OptAttribute = 206,
    SecurityExchange = 207,
    NotifyBrokerOfCredit = 208,
    AllocHandlInst = 209,
    MaxShow = 210,
    PegDifference = 211,
    XmlDataLen = 212,
    XmlData = 213,
    SettlInstRefID = 214,
    NoRoutingIDs = 215,
    RoutingType = 216,
    RoutingID = 217,
    SpreadToBenchmark = 218,
    Benchmark = 219,
    CouponRate = 223,
    ContractMultiplier = 231,
    MDReqID = 262,
    SubscriptionRequestType = 263,
    MarketDepth = 264,
    MDUpdateType = 265,
    AggregatedBook = 266,
    NoMDEntryTypes = 267,
    NoMDEntries = 268,
    MDEntryType = 269,
    MDEntryPx = 270,
    MDEntrySize = 271,
    MDEntryDate = 272,
    MDEntryTime = 273,
    TickDirection = 274,
    MDMkt = 275,
    QuoteCondition = 276,
    TradeCondition = 277,
    MDEntryID = 278,
    MDUpdateAction = 279,
    MDEntryRefID = 280,
    MDReqRejReason = 281,
    MDEntryOriginator = 282,
    LocationID = 283,
    DeskID = 284,
    DeleteReason = 285,
    OpenCloseSettleFlag = 286,
    SellerDays = 287,
    MDEntryBuyer = 288,
    MDEntrySeller = 289,
    MDEntryPositionNo = 290,
    FinancialStatus = 291,
    CorporateAction = 292,
    DefBidSize = 293,
    DefOfferSize = 294,
    NoQuoteEntries = 295,
    NoQuoteSets = 296,
    QuoteAckStatus = 297,
    QuoteCancelType = 298,
    QuoteEntryID = 299,
    QuoteRejectReason = 300,
    QuoteResponseLevel = 301,
    QuoteSetID = 302,
    QuoteRequestType = 303,
    TotQuoteEntries = 304,
    UnderlyingIDSource = 305,
    UnderlyingIssuer = 306,
    UnderlyingSecurityDesc = 307,
    UnderlyingSecurityExchange = 308,
    UnderlyingSecurityID = 309,
    UnderlyingSecurityType = 310,
    UnderlyingSymbol = 311,
    UnderlyingSymbolSfx = 312,
    UnderlyingMaturityMonthYear = 313,
    UnderlyingMaturityDay = 314,
    UnderlyingPutOrCall = 315,
    UnderlyingStrikePrice = 316,
    UnderlyingOptAttribute = 317,
    UnderlyingCurrency = 318,
    RatioQty = 319,
    SecurityReqID = 320,
    SecurityRequestType = 321,
    SecurityResponseID = 322,
    SecurityResponseType = 323,
    SecurityStatusReqID = 324,
    UnsolicitedIndicator = 325,
    SecurityTradingStatus = 326,
    HaltReasonChar = 327,
    InViewOfCommon = 328,
    DueToRelated = 329,
    BuyVolume = 330,
    SellVolume = 331,
    HighPx = 332,
    LowPx = 333,
    Adjustment = 334,
    TradSesReqID = 335,
    TradingSessionID = 336,
    ContraTrader = 337,
    TradSesMethod = 338,
    TradSesMode = 339,
    TradSesStatus = 340,
    TradSesStartTime = 341,
    TradSesOpenTime = 342,
    TradSesPreCloseTime = 343,
    TradSesCloseTime = 344,
    TradSesEndTime = 345,
    NumberOfOrders = 346,
    MessageEncoding = 347,
    EncodedIssuerLen = 348,
    EncodedIssuer = 349,
    EncodedSecurityDescLen = 350,
    EncodedSecurityDesc = 351,
    EncodedListExecInstLen = 352,
    EncodedListExecInst = 353,
    EncodedTextLen = 354,
    EncodedText = 355,
    EncodedSubjectLen = 356,
    EncodedSubject = 357,
    EncodedHeadlineLen = 358,
    EncodedHeadline = 359,
    EncodedAllocTextLen = 360,
    EncodedAllocText = 361,
    EncodedUnderlyingIssuerLen = 362,
    EncodedUnderlyingIssuer = 363,
    EncodedUnderlyingSecurityDescLen = 364,
    EncodedUnderlyingSecurityDesc = 365,
    AllocPrice = 366,
    QuoteSetValidUntilTime = 367,
    QuoteEntryRejectReason = 368,
    LastMsgSeqNumProcessed = 369,
    OnBehalfOfSendingTime = 370,
    RefTagID = 371,
    RefMsgType = 372,
    SessionRejectReason = 373,
    BidRequestTransType = 374,
    ContraBroker = 375,
    ComplianceID = 376,
    SolicitedFlag = 377,
    ExecRestatementReason = 378,
    BusinessRejectRefID = 379,
    BusinessRejectReason = 380,
    GrossTradeAmt = 381,
    NoContraBrokers = 382,
    MaxMessageSize = 383,
    NoMsgTypes = 384,
    MsgDirection = 385,
    NoTradingSessions = 386,
    TotalVolumeTraded = 387,
    DiscretionInst = 388,
    DiscretionOffset = 389,
    BidID = 390,
    ClientBidID = 391,
    ListName = 392,
    TotalNumSecurities = 393,
    BidType = 394,
    NumTickets = 395,
    SideValue1 = 396,
    SideValue2 = 397,
    NoBidDescriptors = 398,
    BidDescriptorType = 399,
    BidDescriptor = 400,
    SideValueInd = 401,
    LiquidityPctLow = 402,
    LiquidityPctHigh = 403,
    LiquidityValue = 404,
    EFPTrackingError = 405,
    FairValue = 406,
    OutsideIndexPct = 407,
    ValueOfFutures = 408,
    LiquidityIndType = 409,
    WtAverageLiquidity = 410,
    ExchangeForPhysical = 411,
    OutMainCntryUIndex = 412,
    CrossPercent = 413,
    ProgRptReqs = 414,
    ProgPeriodInterval = 415,
    IncTaxInd = 416,
    NumBidders = 417,
    TradeType = 418,
    BasisPxType = 419,
    NoBidComponents = 420,
    Country = 421,
    TotNoStrikes = 422,
    PriceType = 423,
    DayOrderQty = 424,
    DayCumQty = 425,
    DayAvgPx = 426,
    GTBookingInst = 427,
    NoStrikes = 428,
    ListStatusType = 429,
    NetGrossInd = 430,
    ListOrderStatus = 431,
    ExpireDate = 432,
    ListExecInstType = 433,
    CxlRejResponseTo = 434,
    UnderlyingCouponRate = 435,
    UnderlyingContractMultiplier = 436,
    ContraTradeQty = 437,
    ContraTradeTime = 438,
    ClearingFirm = 439,
    ClearingAccount = 440,
    LiquidityNumSecurities = 441,
    MultiLegReportingType = 442,
    StrikeTime = 443,
    ListStatusText = 444,
    EncodedListStatusTextLen = 445,
    EncodedListStatusText = 446,
}
impl TryFrom<u32> for Tags {
    type Error = DecodeError;
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        match u {
            1 => Ok(Tags::Account),
            2 => Ok(Tags::AdvId),
            3 => Ok(Tags::AdvRefID),
            4 => Ok(Tags::AdvSide),
            5 => Ok(Tags::AdvTransType),
            6 => Ok(Tags::AvgPx),
            7 => Ok(Tags::BeginSeqNo),
            8 => Ok(Tags::BeginString),
            9 => Ok(Tags::BodyLength),
            10 => Ok(Tags::CheckSum),
            11 => Ok(Tags::ClOrdID),
            12 => Ok(Tags::Commission),
            13 => Ok(Tags::CommType),
            14 => Ok(Tags::CumQty),
            15 => Ok(Tags::Currency),
            16 => Ok(Tags::EndSeqNo),
            17 => Ok(Tags::ExecID),
            18 => Ok(Tags::ExecInst),
            19 => Ok(Tags::ExecRefID),
            20 => Ok(Tags::ExecTransType),
            21 => Ok(Tags::HandlInst),
            22 => Ok(Tags::IDSource),
            23 => Ok(Tags::IOIid),
            24 => Ok(Tags::IOIOthSvc),
            25 => Ok(Tags::IOIQltyInd),
            26 => Ok(Tags::IOIRefID),
            27 => Ok(Tags::IOIShares),
            28 => Ok(Tags::IOITransType),
            29 => Ok(Tags::LastCapacity),
            30 => Ok(Tags::LastMkt),
            31 => Ok(Tags::LastPx),
            32 => Ok(Tags::LastShares),
            33 => Ok(Tags::LinesOfText),
            34 => Ok(Tags::MsgSeqNum),
            35 => Ok(Tags::MsgType),
            36 => Ok(Tags::NewSeqNo),
            37 => Ok(Tags::OrderID),
            38 => Ok(Tags::OrderQty),
            39 => Ok(Tags::OrdStatus),
            40 => Ok(Tags::OrdType),
            41 => Ok(Tags::OrigClOrdID),
            42 => Ok(Tags::OrigTime),
            43 => Ok(Tags::PossDupFlag),
            44 => Ok(Tags::Price),
            45 => Ok(Tags::RefSeqNum),
            46 => Ok(Tags::RelatdSym),
            47 => Ok(Tags::Rule80A),
            48 => Ok(Tags::SecurityID),
            49 => Ok(Tags::SenderCompID),
            50 => Ok(Tags::SenderSubID),
            51 => Ok(Tags::SendingDate),
            52 => Ok(Tags::SendingTime),
            53 => Ok(Tags::Shares),
            54 => Ok(Tags::Side),
            55 => Ok(Tags::Symbol),
            56 => Ok(Tags::TargetCompID),
            57 => Ok(Tags::TargetSubID),
            58 => Ok(Tags::Text),
            59 => Ok(Tags::TimeInForce),
            60 => Ok(Tags::TransactTime),
            61 => Ok(Tags::Urgency),
            62 => Ok(Tags::ValidUntilTime),
            63 => Ok(Tags::SettlmntTyp),
            64 => Ok(Tags::FutSettDate),
            65 => Ok(Tags::SymbolSfx),
            66 => Ok(Tags::ListID),
            67 => Ok(Tags::ListSeqNo),
            68 => Ok(Tags::TotNoOrders),
            69 => Ok(Tags::ListExecInst),
            70 => Ok(Tags::AllocID),
            71 => Ok(Tags::AllocTransType),
            72 => Ok(Tags::RefAllocID),
            73 => Ok(Tags::NoOrders),
            74 => Ok(Tags::AvgPrxPrecision),
            75 => Ok(Tags::TradeDate),
            76 => Ok(Tags::ExecBroker),
            77 => Ok(Tags::OpenClose),
            78 => Ok(Tags::NoAllocs),
            79 => Ok(Tags::AllocAccount),
            80 => Ok(Tags::AllocShares),
            81 => Ok(Tags::ProcessCode),
            82 => Ok(Tags::NoRpts),
            83 => Ok(Tags::RptSeq),
            84 => Ok(Tags::CxlQty),
            85 => Ok(Tags::NoDlvyInst),
            86 => Ok(Tags::DlvyInst),
            87 => Ok(Tags::AllocStatus),
            88 => Ok(Tags::AllocRejCode),
            89 => Ok(Tags::Signature),
            90 => Ok(Tags::SecureDataLen),
            91 => Ok(Tags::SecureData),
            92 => Ok(Tags::BrokerOfCredit),
            93 => Ok(Tags::SignatureLength),
            94 => Ok(Tags::EmailType),
            95 => Ok(Tags::RawDataLength),
            96 => Ok(Tags::RawData),
            97 => Ok(Tags::PossResend),
            98 => Ok(Tags::EncryptMethod),
            99 => Ok(Tags::StopPx),
            100 => Ok(Tags::ExDestination),
            102 => Ok(Tags::CxlRejReason),
            103 => Ok(Tags::OrdRejReason),
            104 => Ok(Tags::IOIQualifier),
            105 => Ok(Tags::WaveNo),
            106 => Ok(Tags::Issuer),
            107 => Ok(Tags::SecurityDesc),
            108 => Ok(Tags::HeartBtInt),
            109 => Ok(Tags::ClientID),
            110 => Ok(Tags::MinQty),
            111 => Ok(Tags::MaxFloor),
            112 => Ok(Tags::TestReqID),
            113 => Ok(Tags::ReportToExch),
            114 => Ok(Tags::LocateReqd),
            115 => Ok(Tags::OnBehalfOfCompID),
            116 => Ok(Tags::OnBehalfOfSubID),
            117 => Ok(Tags::QuoteID),
            118 => Ok(Tags::NetMoney),
            119 => Ok(Tags::SettlCurrAmt),
            120 => Ok(Tags::SettlCurrency),
            121 => Ok(Tags::ForexReq),
            122 => Ok(Tags::OrigSendingTime),
            123 => Ok(Tags::GapFillFlag),
            124 => Ok(Tags::NoExecs),
            125 => Ok(Tags::CxlType),
            126 => Ok(Tags::ExpireTime),
            127 => Ok(Tags::DKReason),
            128 => Ok(Tags::DeliverToCompID),
            129 => Ok(Tags::DeliverToSubID),
            130 => Ok(Tags::IOINaturalFlag),
            131 => Ok(Tags::QuoteReqID),
            132 => Ok(Tags::BidPx),
            133 => Ok(Tags::OfferPx),
            134 => Ok(Tags::BidSize),
            135 => Ok(Tags::OfferSize),
            136 => Ok(Tags::NoMiscFees),
            137 => Ok(Tags::MiscFeeAmt),
            138 => Ok(Tags::MiscFeeCurr),
            139 => Ok(Tags::MiscFeeType),
            140 => Ok(Tags::PrevClosePx),
            141 => Ok(Tags::ResetSeqNumFlag),
            142 => Ok(Tags::SenderLocationID),
            143 => Ok(Tags::TargetLocationID),
            144 => Ok(Tags::OnBehalfOfLocationID),
            145 => Ok(Tags::DeliverToLocationID),
            146 => Ok(Tags::NoRelatedSym),
            147 => Ok(Tags::Subject),
            148 => Ok(Tags::Headline),
            149 => Ok(Tags::URLLink),
            150 => Ok(Tags::ExecType),
            151 => Ok(Tags::LeavesQty),
            152 => Ok(Tags::CashOrderQty),
            153 => Ok(Tags::AllocAvgPx),
            154 => Ok(Tags::AllocNetMoney),
            155 => Ok(Tags::SettlCurrFxRate),
            156 => Ok(Tags::SettlCurrFxRateCalc),
            157 => Ok(Tags::NumDaysInterest),
            158 => Ok(Tags::AccruedInterestRate),
            159 => Ok(Tags::AccruedInterestAmt),
            160 => Ok(Tags::SettlInstMode),
            161 => Ok(Tags::AllocText),
            162 => Ok(Tags::SettlInstID),
            163 => Ok(Tags::SettlInstTransType),
            164 => Ok(Tags::EmailThreadID),
            165 => Ok(Tags::SettlInstSource),
            166 => Ok(Tags::SettlLocation),
            167 => Ok(Tags::SecurityType),
            168 => Ok(Tags::EffectiveTime),
            169 => Ok(Tags::StandInstDbType),
            170 => Ok(Tags::StandInstDbName),
            171 => Ok(Tags::StandInstDbID),
            172 => Ok(Tags::SettlDeliveryType),
            173 => Ok(Tags::SettlDepositoryCode),
            174 => Ok(Tags::SettlBrkrCode),
            175 => Ok(Tags::SettlInstCode),
            176 => Ok(Tags::SecuritySettlAgentName),
            177 => Ok(Tags::SecuritySettlAgentCode),
            178 => Ok(Tags::SecuritySettlAgentAcctNum),
            179 => Ok(Tags::SecuritySettlAgentAcctName),
            180 => Ok(Tags::SecuritySettlAgentContactName),
            181 => Ok(Tags::SecuritySettlAgentContactPhone),
            182 => Ok(Tags::CashSettlAgentName),
            183 => Ok(Tags::CashSettlAgentCode),
            184 => Ok(Tags::CashSettlAgentAcctNum),
            185 => Ok(Tags::CashSettlAgentAcctName),
            186 => Ok(Tags::CashSettlAgentContactName),
            187 => Ok(Tags::CashSettlAgentContactPhone),
            188 => Ok(Tags::BidSpotRate),
            189 => Ok(Tags::BidForwardPoints),
            190 => Ok(Tags::OfferSpotRate),
            191 => Ok(Tags::OfferForwardPoints),
            192 => Ok(Tags::OrderQty2),
            193 => Ok(Tags::FutSettDate2),
            194 => Ok(Tags::LastSpotRate),
            195 => Ok(Tags::LastForwardPoints),
            196 => Ok(Tags::AllocLinkID),
            197 => Ok(Tags::AllocLinkType),
            198 => Ok(Tags::SecondaryOrderID),
            199 => Ok(Tags::NoIOIQualifiers),
            200 => Ok(Tags::MaturityMonthYear),
            201 => Ok(Tags::PutOrCall),
            202 => Ok(Tags::StrikePrice),
            203 => Ok(Tags::CoveredOrUncovered),
            204 => Ok(Tags::CustomerOrFirm),
            205 => Ok(Tags::MaturityDay),
            206 => Ok(Tags::OptAttribute),
            207 => Ok(Tags::SecurityExchange),
            208 => Ok(Tags::NotifyBrokerOfCredit),
            209 => Ok(Tags::AllocHandlInst),
            210 => Ok(Tags::MaxShow),
            211 => Ok(Tags::PegDifference),
            212 => Ok(Tags::XmlDataLen),
            213 => Ok(Tags::XmlData),
            214 => Ok(Tags::SettlInstRefID),
            215 => Ok(Tags::NoRoutingIDs),
            216 => Ok(Tags::RoutingType),
            217 => Ok(Tags::RoutingID),
            218 => Ok(Tags::SpreadToBenchmark),
            219 => Ok(Tags::Benchmark),
            223 => Ok(Tags::CouponRate),
            231 => Ok(Tags::ContractMultiplier),
            262 => Ok(Tags::MDReqID),
            263 => Ok(Tags::SubscriptionRequestType),
            264 => Ok(Tags::MarketDepth),
            265 => Ok(Tags::MDUpdateType),
            266 => Ok(Tags::AggregatedBook),
            267 => Ok(Tags::NoMDEntryTypes),
            268 => Ok(Tags::NoMDEntries),
            269 => Ok(Tags::MDEntryType),
            270 => Ok(Tags::MDEntryPx),
            271 => Ok(Tags::MDEntrySize),
            272 => Ok(Tags::MDEntryDate),
            273 => Ok(Tags::MDEntryTime),
            274 => Ok(Tags::TickDirection),
            275 => Ok(Tags::MDMkt),
            276 => Ok(Tags::QuoteCondition),
            277 => Ok(Tags::TradeCondition),
            278 => Ok(Tags::MDEntryID),
            279 => Ok(Tags::MDUpdateAction),
            280 => Ok(Tags::MDEntryRefID),
            281 => Ok(Tags::MDReqRejReason),
            282 => Ok(Tags::MDEntryOriginator),
            283 => Ok(Tags::LocationID),
            284 => Ok(Tags::DeskID),
            285 => Ok(Tags::DeleteReason),
            286 => Ok(Tags::OpenCloseSettleFlag),
            287 => Ok(Tags::SellerDays),
            288 => Ok(Tags::MDEntryBuyer),
            289 => Ok(Tags::MDEntrySeller),
            290 => Ok(Tags::MDEntryPositionNo),
            291 => Ok(Tags::FinancialStatus),
            292 => Ok(Tags::CorporateAction),
            293 => Ok(Tags::DefBidSize),
            294 => Ok(Tags::DefOfferSize),
            295 => Ok(Tags::NoQuoteEntries),
            296 => Ok(Tags::NoQuoteSets),
            297 => Ok(Tags::QuoteAckStatus),
            298 => Ok(Tags::QuoteCancelType),
            299 => Ok(Tags::QuoteEntryID),
            300 => Ok(Tags::QuoteRejectReason),
            301 => Ok(Tags::QuoteResponseLevel),
            302 => Ok(Tags::QuoteSetID),
            303 => Ok(Tags::QuoteRequestType),
            304 => Ok(Tags::TotQuoteEntries),
            305 => Ok(Tags::UnderlyingIDSource),
            306 => Ok(Tags::UnderlyingIssuer),
            307 => Ok(Tags::UnderlyingSecurityDesc),
            308 => Ok(Tags::UnderlyingSecurityExchange),
            309 => Ok(Tags::UnderlyingSecurityID),
            310 => Ok(Tags::UnderlyingSecurityType),
            311 => Ok(Tags::UnderlyingSymbol),
            312 => Ok(Tags::UnderlyingSymbolSfx),
            313 => Ok(Tags::UnderlyingMaturityMonthYear),
            314 => Ok(Tags::UnderlyingMaturityDay),
            315 => Ok(Tags::UnderlyingPutOrCall),
            316 => Ok(Tags::UnderlyingStrikePrice),
            317 => Ok(Tags::UnderlyingOptAttribute),
            318 => Ok(Tags::UnderlyingCurrency),
            319 => Ok(Tags::RatioQty),
            320 => Ok(Tags::SecurityReqID),
            321 => Ok(Tags::SecurityRequestType),
            322 => Ok(Tags::SecurityResponseID),
            323 => Ok(Tags::SecurityResponseType),
            324 => Ok(Tags::SecurityStatusReqID),
            325 => Ok(Tags::UnsolicitedIndicator),
            326 => Ok(Tags::SecurityTradingStatus),
            327 => Ok(Tags::HaltReasonChar),
            328 => Ok(Tags::InViewOfCommon),
            329 => Ok(Tags::DueToRelated),
            330 => Ok(Tags::BuyVolume),
            331 => Ok(Tags::SellVolume),
            332 => Ok(Tags::HighPx),
            333 => Ok(Tags::LowPx),
            334 => Ok(Tags::Adjustment),
            335 => Ok(Tags::TradSesReqID),
            336 => Ok(Tags::TradingSessionID),
            337 => Ok(Tags::ContraTrader),
            338 => Ok(Tags::TradSesMethod),
            339 => Ok(Tags::TradSesMode),
            340 => Ok(Tags::TradSesStatus),
            341 => Ok(Tags::TradSesStartTime),
            342 => Ok(Tags::TradSesOpenTime),
            343 => Ok(Tags::TradSesPreCloseTime),
            344 => Ok(Tags::TradSesCloseTime),
            345 => Ok(Tags::TradSesEndTime),
            346 => Ok(Tags::NumberOfOrders),
            347 => Ok(Tags::MessageEncoding),
            348 => Ok(Tags::EncodedIssuerLen),
            349 => Ok(Tags::EncodedIssuer),
            350 => Ok(Tags::EncodedSecurityDescLen),
            351 => Ok(Tags::EncodedSecurityDesc),
            352 => Ok(Tags::EncodedListExecInstLen),
            353 => Ok(Tags::EncodedListExecInst),
            354 => Ok(Tags::EncodedTextLen),
            355 => Ok(Tags::EncodedText),
            356 => Ok(Tags::EncodedSubjectLen),
            357 => Ok(Tags::EncodedSubject),
            358 => Ok(Tags::EncodedHeadlineLen),
            359 => Ok(Tags::EncodedHeadline),
            360 => Ok(Tags::EncodedAllocTextLen),
            361 => Ok(Tags::EncodedAllocText),
            362 => Ok(Tags::EncodedUnderlyingIssuerLen),
            363 => Ok(Tags::EncodedUnderlyingIssuer),
            364 => Ok(Tags::EncodedUnderlyingSecurityDescLen),
            365 => Ok(Tags::EncodedUnderlyingSecurityDesc),
            366 => Ok(Tags::AllocPrice),
            367 => Ok(Tags::QuoteSetValidUntilTime),
            368 => Ok(Tags::QuoteEntryRejectReason),
            369 => Ok(Tags::LastMsgSeqNumProcessed),
            370 => Ok(Tags::OnBehalfOfSendingTime),
            371 => Ok(Tags::RefTagID),
            372 => Ok(Tags::RefMsgType),
            373 => Ok(Tags::SessionRejectReason),
            374 => Ok(Tags::BidRequestTransType),
            375 => Ok(Tags::ContraBroker),
            376 => Ok(Tags::ComplianceID),
            377 => Ok(Tags::SolicitedFlag),
            378 => Ok(Tags::ExecRestatementReason),
            379 => Ok(Tags::BusinessRejectRefID),
            380 => Ok(Tags::BusinessRejectReason),
            381 => Ok(Tags::GrossTradeAmt),
            382 => Ok(Tags::NoContraBrokers),
            383 => Ok(Tags::MaxMessageSize),
            384 => Ok(Tags::NoMsgTypes),
            385 => Ok(Tags::MsgDirection),
            386 => Ok(Tags::NoTradingSessions),
            387 => Ok(Tags::TotalVolumeTraded),
            388 => Ok(Tags::DiscretionInst),
            389 => Ok(Tags::DiscretionOffset),
            390 => Ok(Tags::BidID),
            391 => Ok(Tags::ClientBidID),
            392 => Ok(Tags::ListName),
            393 => Ok(Tags::TotalNumSecurities),
            394 => Ok(Tags::BidType),
            395 => Ok(Tags::NumTickets),
            396 => Ok(Tags::SideValue1),
            397 => Ok(Tags::SideValue2),
            398 => Ok(Tags::NoBidDescriptors),
            399 => Ok(Tags::BidDescriptorType),
            400 => Ok(Tags::BidDescriptor),
            401 => Ok(Tags::SideValueInd),
            402 => Ok(Tags::LiquidityPctLow),
            403 => Ok(Tags::LiquidityPctHigh),
            404 => Ok(Tags::LiquidityValue),
            405 => Ok(Tags::EFPTrackingError),
            406 => Ok(Tags::FairValue),
            407 => Ok(Tags::OutsideIndexPct),
            408 => Ok(Tags::ValueOfFutures),
            409 => Ok(Tags::LiquidityIndType),
            410 => Ok(Tags::WtAverageLiquidity),
            411 => Ok(Tags::ExchangeForPhysical),
            412 => Ok(Tags::OutMainCntryUIndex),
            413 => Ok(Tags::CrossPercent),
            414 => Ok(Tags::ProgRptReqs),
            415 => Ok(Tags::ProgPeriodInterval),
            416 => Ok(Tags::IncTaxInd),
            417 => Ok(Tags::NumBidders),
            418 => Ok(Tags::TradeType),
            419 => Ok(Tags::BasisPxType),
            420 => Ok(Tags::NoBidComponents),
            421 => Ok(Tags::Country),
            422 => Ok(Tags::TotNoStrikes),
            423 => Ok(Tags::PriceType),
            424 => Ok(Tags::DayOrderQty),
            425 => Ok(Tags::DayCumQty),
            426 => Ok(Tags::DayAvgPx),
            427 => Ok(Tags::GTBookingInst),
            428 => Ok(Tags::NoStrikes),
            429 => Ok(Tags::ListStatusType),
            430 => Ok(Tags::NetGrossInd),
            431 => Ok(Tags::ListOrderStatus),
            432 => Ok(Tags::ExpireDate),
            433 => Ok(Tags::ListExecInstType),
            434 => Ok(Tags::CxlRejResponseTo),
            435 => Ok(Tags::UnderlyingCouponRate),
            436 => Ok(Tags::UnderlyingContractMultiplier),
            437 => Ok(Tags::ContraTradeQty),
            438 => Ok(Tags::ContraTradeTime),
            439 => Ok(Tags::ClearingFirm),
            440 => Ok(Tags::ClearingAccount),
            441 => Ok(Tags::LiquidityNumSecurities),
            442 => Ok(Tags::MultiLegReportingType),
            443 => Ok(Tags::StrikeTime),
            444 => Ok(Tags::ListStatusText),
            445 => Ok(Tags::EncodedListStatusTextLen),
            446 => Ok(Tags::EncodedListStatusText),
            _ => Err(DecodeError::UnknownTag(u)),
        }
    }
}
impl From<Tags> for u32 {
    fn from(value: Tags) -> u32 {
        value as isize as u32
    }
}

pub fn get_data_ref(tag: u32) -> Option<u32> {
    match tag {
        93 => Some(89),
        90 => Some(91),
        95 => Some(96),
        212 => Some(213),
        348 => Some(349),
        350 => Some(351),
        352 => Some(353),
        354 => Some(355),
        356 => Some(357),
        358 => Some(359),
        360 => Some(361),
        362 => Some(363),
        364 => Some(365),
        445 => Some(446),

        _ => None,
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum AdvSide {
    BUY = 'B' as isize,
    SELL = 'S' as isize,
    TRADE = 'T' as isize,
    CROSS = 'X' as isize,
}

impl From<AdvSide> for char {
    fn from(a: AdvSide) -> char {
        a as isize as u8 as char
    }
}

impl From<AdvSide> for &'static [u8] {
    fn from(a: AdvSide) -> &'static [u8] {
        match a {
            AdvSide::BUY => b"B",
            AdvSide::SELL => b"S",
            AdvSide::TRADE => b"T",
            AdvSide::CROSS => b"X",
        }
    }
}

impl TryFrom<char> for AdvSide {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'B' => Ok(Self::BUY),
            'S' => Ok(Self::SELL),
            'T' => Ok(Self::TRADE),
            'X' => Ok(Self::CROSS),
            _ => Err(DecodeError::UnknownChar(Tags::AdvSide, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum CommType {
    PER_SHARE = '1' as isize,
    PERCENTAGE = '2' as isize,
    ABSOLUTE = '3' as isize,
}

impl From<CommType> for char {
    fn from(a: CommType) -> char {
        a as isize as u8 as char
    }
}

impl From<CommType> for &'static [u8] {
    fn from(a: CommType) -> &'static [u8] {
        match a {
            CommType::PER_SHARE => b"1",
            CommType::PERCENTAGE => b"2",
            CommType::ABSOLUTE => b"3",
        }
    }
}

impl TryFrom<char> for CommType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Self::PER_SHARE),
            '2' => Ok(Self::PERCENTAGE),
            '3' => Ok(Self::ABSOLUTE),
            _ => Err(DecodeError::UnknownChar(Tags::CommType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum ExecTransType {
    NEW = '0' as isize,
    CANCEL = '1' as isize,
    CORRECT = '2' as isize,
    STATUS = '3' as isize,
}

impl From<ExecTransType> for char {
    fn from(a: ExecTransType) -> char {
        a as isize as u8 as char
    }
}

impl From<ExecTransType> for &'static [u8] {
    fn from(a: ExecTransType) -> &'static [u8] {
        match a {
            ExecTransType::NEW => b"0",
            ExecTransType::CANCEL => b"1",
            ExecTransType::CORRECT => b"2",
            ExecTransType::STATUS => b"3",
        }
    }
}

impl TryFrom<char> for ExecTransType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::NEW),
            '1' => Ok(Self::CANCEL),
            '2' => Ok(Self::CORRECT),
            '3' => Ok(Self::STATUS),
            _ => Err(DecodeError::UnknownChar(Tags::ExecTransType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum HandlInst {
    AUTOMATED_EXECUTION_ORDER_PRIVATE_NO_BROKER_INTERVENTION = '1' as isize,
    AUTOMATED_EXECUTION_ORDER_PUBLIC_BROKER_INTERVENTION_OK = '2' as isize,
    MANUAL_ORDER_BEST_EXECUTION = '3' as isize,
}

impl From<HandlInst> for char {
    fn from(a: HandlInst) -> char {
        a as isize as u8 as char
    }
}

impl From<HandlInst> for &'static [u8] {
    fn from(a: HandlInst) -> &'static [u8] {
        match a {
            HandlInst::AUTOMATED_EXECUTION_ORDER_PRIVATE_NO_BROKER_INTERVENTION => b"1",
            HandlInst::AUTOMATED_EXECUTION_ORDER_PUBLIC_BROKER_INTERVENTION_OK => b"2",
            HandlInst::MANUAL_ORDER_BEST_EXECUTION => b"3",
        }
    }
}

impl TryFrom<char> for HandlInst {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Self::AUTOMATED_EXECUTION_ORDER_PRIVATE_NO_BROKER_INTERVENTION),
            '2' => Ok(Self::AUTOMATED_EXECUTION_ORDER_PUBLIC_BROKER_INTERVENTION_OK),
            '3' => Ok(Self::MANUAL_ORDER_BEST_EXECUTION),
            _ => Err(DecodeError::UnknownChar(Tags::HandlInst, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum IOIQltyInd {
    HIGH = 'H' as isize,
    LOW = 'L' as isize,
    MEDIUM = 'M' as isize,
}

impl From<IOIQltyInd> for char {
    fn from(a: IOIQltyInd) -> char {
        a as isize as u8 as char
    }
}

impl From<IOIQltyInd> for &'static [u8] {
    fn from(a: IOIQltyInd) -> &'static [u8] {
        match a {
            IOIQltyInd::HIGH => b"H",
            IOIQltyInd::LOW => b"L",
            IOIQltyInd::MEDIUM => b"M",
        }
    }
}

impl TryFrom<char> for IOIQltyInd {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'H' => Ok(Self::HIGH),
            'L' => Ok(Self::LOW),
            'M' => Ok(Self::MEDIUM),
            _ => Err(DecodeError::UnknownChar(Tags::IOIQltyInd, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum IOITransType {
    CANCEL = 'C' as isize,
    NEW = 'N' as isize,
    REPLACE = 'R' as isize,
}

impl From<IOITransType> for char {
    fn from(a: IOITransType) -> char {
        a as isize as u8 as char
    }
}

impl From<IOITransType> for &'static [u8] {
    fn from(a: IOITransType) -> &'static [u8] {
        match a {
            IOITransType::CANCEL => b"C",
            IOITransType::NEW => b"N",
            IOITransType::REPLACE => b"R",
        }
    }
}

impl TryFrom<char> for IOITransType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'C' => Ok(Self::CANCEL),
            'N' => Ok(Self::NEW),
            'R' => Ok(Self::REPLACE),
            _ => Err(DecodeError::UnknownChar(Tags::IOITransType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum LastCapacity {
    AGENT = '1' as isize,
    CROSS_AS_AGENT = '2' as isize,
    CROSS_AS_PRINCIPAL = '3' as isize,
    PRINCIPAL = '4' as isize,
}

impl From<LastCapacity> for char {
    fn from(a: LastCapacity) -> char {
        a as isize as u8 as char
    }
}

impl From<LastCapacity> for &'static [u8] {
    fn from(a: LastCapacity) -> &'static [u8] {
        match a {
            LastCapacity::AGENT => b"1",
            LastCapacity::CROSS_AS_AGENT => b"2",
            LastCapacity::CROSS_AS_PRINCIPAL => b"3",
            LastCapacity::PRINCIPAL => b"4",
        }
    }
}

impl TryFrom<char> for LastCapacity {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Self::AGENT),
            '2' => Ok(Self::CROSS_AS_AGENT),
            '3' => Ok(Self::CROSS_AS_PRINCIPAL),
            '4' => Ok(Self::PRINCIPAL),
            _ => Err(DecodeError::UnknownChar(Tags::LastCapacity, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum MsgType {
    HEARTBEAT = '0' as isize,
    TEST_REQUEST = '1' as isize,
    RESEND_REQUEST = '2' as isize,
    REJECT = '3' as isize,
    SEQUENCE_RESET = '4' as isize,
    LOGOUT = '5' as isize,
    INDICATION_OF_INTEREST = '6' as isize,
    ADVERTISEMENT = '7' as isize,
    EXECUTION_REPORT = '8' as isize,
    ORDER_CANCEL_REJECT = '9' as isize,
    QUOTE_STATUS_REQUEST = 'a' as isize,
    LOGON = 'A' as isize,
    NEWS = 'B' as isize,
    QUOTE_ACKNOWLEDGEMENT = 'b' as isize,
    EMAIL = 'C' as isize,
    SECURITY_DEFINITION_REQUEST = 'c' as isize,
    ORDER_SINGLE = 'D' as isize,
    SECURITY_DEFINITION = 'd' as isize,
    ORDER_LIST = 'E' as isize,
    SECURITY_STATUS_REQUEST = 'e' as isize,
    SECURITY_STATUS = 'f' as isize,
    ORDER_CANCEL_REQUEST = 'F' as isize,
    ORDER_CANCEL_REPLACE_REQUEST = 'G' as isize,
    TRADING_SESSION_STATUS_REQUEST = 'g' as isize,
    ORDER_STATUS_REQUEST = 'H' as isize,
    TRADING_SESSION_STATUS = 'h' as isize,
    MASS_QUOTE = 'i' as isize,
    BUSINESS_MESSAGE_REJECT = 'j' as isize,
    ALLOCATION = 'J' as isize,
    LIST_CANCEL_REQUEST = 'K' as isize,
    BID_REQUEST = 'k' as isize,
    BID_RESPONSE = 'l' as isize,
    LIST_EXECUTE = 'L' as isize,
    LIST_STRIKE_PRICE = 'm' as isize,
    LIST_STATUS_REQUEST = 'M' as isize,
    LIST_STATUS = 'N' as isize,
    ALLOCATION_ACK = 'P' as isize,
    DONT_KNOW_TRADE = 'Q' as isize,
    QUOTE_REQUEST = 'R' as isize,
    QUOTE = 'S' as isize,
    SETTLEMENT_INSTRUCTIONS = 'T' as isize,
    MARKET_DATA_REQUEST = 'V' as isize,
    MARKET_DATA_SNAPSHOT_FULL_REFRESH = 'W' as isize,
    MARKET_DATA_INCREMENTAL_REFRESH = 'X' as isize,
    MARKET_DATA_REQUEST_REJECT = 'Y' as isize,
    QUOTE_CANCEL = 'Z' as isize,
}

impl From<MsgType> for char {
    fn from(a: MsgType) -> char {
        a as isize as u8 as char
    }
}

impl From<MsgType> for &'static [u8] {
    fn from(a: MsgType) -> &'static [u8] {
        match a {
            MsgType::HEARTBEAT => b"0",
            MsgType::TEST_REQUEST => b"1",
            MsgType::RESEND_REQUEST => b"2",
            MsgType::REJECT => b"3",
            MsgType::SEQUENCE_RESET => b"4",
            MsgType::LOGOUT => b"5",
            MsgType::INDICATION_OF_INTEREST => b"6",
            MsgType::ADVERTISEMENT => b"7",
            MsgType::EXECUTION_REPORT => b"8",
            MsgType::ORDER_CANCEL_REJECT => b"9",
            MsgType::QUOTE_STATUS_REQUEST => b"a",
            MsgType::LOGON => b"A",
            MsgType::NEWS => b"B",
            MsgType::QUOTE_ACKNOWLEDGEMENT => b"b",
            MsgType::EMAIL => b"C",
            MsgType::SECURITY_DEFINITION_REQUEST => b"c",
            MsgType::ORDER_SINGLE => b"D",
            MsgType::SECURITY_DEFINITION => b"d",
            MsgType::ORDER_LIST => b"E",
            MsgType::SECURITY_STATUS_REQUEST => b"e",
            MsgType::SECURITY_STATUS => b"f",
            MsgType::ORDER_CANCEL_REQUEST => b"F",
            MsgType::ORDER_CANCEL_REPLACE_REQUEST => b"G",
            MsgType::TRADING_SESSION_STATUS_REQUEST => b"g",
            MsgType::ORDER_STATUS_REQUEST => b"H",
            MsgType::TRADING_SESSION_STATUS => b"h",
            MsgType::MASS_QUOTE => b"i",
            MsgType::BUSINESS_MESSAGE_REJECT => b"j",
            MsgType::ALLOCATION => b"J",
            MsgType::LIST_CANCEL_REQUEST => b"K",
            MsgType::BID_REQUEST => b"k",
            MsgType::BID_RESPONSE => b"l",
            MsgType::LIST_EXECUTE => b"L",
            MsgType::LIST_STRIKE_PRICE => b"m",
            MsgType::LIST_STATUS_REQUEST => b"M",
            MsgType::LIST_STATUS => b"N",
            MsgType::ALLOCATION_ACK => b"P",
            MsgType::DONT_KNOW_TRADE => b"Q",
            MsgType::QUOTE_REQUEST => b"R",
            MsgType::QUOTE => b"S",
            MsgType::SETTLEMENT_INSTRUCTIONS => b"T",
            MsgType::MARKET_DATA_REQUEST => b"V",
            MsgType::MARKET_DATA_SNAPSHOT_FULL_REFRESH => b"W",
            MsgType::MARKET_DATA_INCREMENTAL_REFRESH => b"X",
            MsgType::MARKET_DATA_REQUEST_REJECT => b"Y",
            MsgType::QUOTE_CANCEL => b"Z",
        }
    }
}

impl TryFrom<char> for MsgType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::HEARTBEAT),
            '1' => Ok(Self::TEST_REQUEST),
            '2' => Ok(Self::RESEND_REQUEST),
            '3' => Ok(Self::REJECT),
            '4' => Ok(Self::SEQUENCE_RESET),
            '5' => Ok(Self::LOGOUT),
            '6' => Ok(Self::INDICATION_OF_INTEREST),
            '7' => Ok(Self::ADVERTISEMENT),
            '8' => Ok(Self::EXECUTION_REPORT),
            '9' => Ok(Self::ORDER_CANCEL_REJECT),
            'a' => Ok(Self::QUOTE_STATUS_REQUEST),
            'A' => Ok(Self::LOGON),
            'B' => Ok(Self::NEWS),
            'b' => Ok(Self::QUOTE_ACKNOWLEDGEMENT),
            'C' => Ok(Self::EMAIL),
            'c' => Ok(Self::SECURITY_DEFINITION_REQUEST),
            'D' => Ok(Self::ORDER_SINGLE),
            'd' => Ok(Self::SECURITY_DEFINITION),
            'E' => Ok(Self::ORDER_LIST),
            'e' => Ok(Self::SECURITY_STATUS_REQUEST),
            'f' => Ok(Self::SECURITY_STATUS),
            'F' => Ok(Self::ORDER_CANCEL_REQUEST),
            'G' => Ok(Self::ORDER_CANCEL_REPLACE_REQUEST),
            'g' => Ok(Self::TRADING_SESSION_STATUS_REQUEST),
            'H' => Ok(Self::ORDER_STATUS_REQUEST),
            'h' => Ok(Self::TRADING_SESSION_STATUS),
            'i' => Ok(Self::MASS_QUOTE),
            'j' => Ok(Self::BUSINESS_MESSAGE_REJECT),
            'J' => Ok(Self::ALLOCATION),
            'K' => Ok(Self::LIST_CANCEL_REQUEST),
            'k' => Ok(Self::BID_REQUEST),
            'l' => Ok(Self::BID_RESPONSE),
            'L' => Ok(Self::LIST_EXECUTE),
            'm' => Ok(Self::LIST_STRIKE_PRICE),
            'M' => Ok(Self::LIST_STATUS_REQUEST),
            'N' => Ok(Self::LIST_STATUS),
            'P' => Ok(Self::ALLOCATION_ACK),
            'Q' => Ok(Self::DONT_KNOW_TRADE),
            'R' => Ok(Self::QUOTE_REQUEST),
            'S' => Ok(Self::QUOTE),
            'T' => Ok(Self::SETTLEMENT_INSTRUCTIONS),
            'V' => Ok(Self::MARKET_DATA_REQUEST),
            'W' => Ok(Self::MARKET_DATA_SNAPSHOT_FULL_REFRESH),
            'X' => Ok(Self::MARKET_DATA_INCREMENTAL_REFRESH),
            'Y' => Ok(Self::MARKET_DATA_REQUEST_REJECT),
            'Z' => Ok(Self::QUOTE_CANCEL),
            _ => Err(DecodeError::UnknownChar(Tags::MsgType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum OrdStatus {
    NEW = '0' as isize,
    PARTIALLY_FILLED = '1' as isize,
    FILLED = '2' as isize,
    DONE_FOR_DAY = '3' as isize,
    CANCELED = '4' as isize,
    REPLACED = '5' as isize,
    PENDING_CANCEL = '6' as isize,
    STOPPED = '7' as isize,
    REJECTED = '8' as isize,
    SUSPENDED = '9' as isize,
    PENDING_NEW = 'A' as isize,
    CALCULATED = 'B' as isize,
    EXPIRED = 'C' as isize,
    ACCEPTED_FOR_BIDDING = 'D' as isize,
    PENDING_REPLACE = 'E' as isize,
}

impl From<OrdStatus> for char {
    fn from(a: OrdStatus) -> char {
        a as isize as u8 as char
    }
}

impl From<OrdStatus> for &'static [u8] {
    fn from(a: OrdStatus) -> &'static [u8] {
        match a {
            OrdStatus::NEW => b"0",
            OrdStatus::PARTIALLY_FILLED => b"1",
            OrdStatus::FILLED => b"2",
            OrdStatus::DONE_FOR_DAY => b"3",
            OrdStatus::CANCELED => b"4",
            OrdStatus::REPLACED => b"5",
            OrdStatus::PENDING_CANCEL => b"6",
            OrdStatus::STOPPED => b"7",
            OrdStatus::REJECTED => b"8",
            OrdStatus::SUSPENDED => b"9",
            OrdStatus::PENDING_NEW => b"A",
            OrdStatus::CALCULATED => b"B",
            OrdStatus::EXPIRED => b"C",
            OrdStatus::ACCEPTED_FOR_BIDDING => b"D",
            OrdStatus::PENDING_REPLACE => b"E",
        }
    }
}

impl TryFrom<char> for OrdStatus {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::NEW),
            '1' => Ok(Self::PARTIALLY_FILLED),
            '2' => Ok(Self::FILLED),
            '3' => Ok(Self::DONE_FOR_DAY),
            '4' => Ok(Self::CANCELED),
            '5' => Ok(Self::REPLACED),
            '6' => Ok(Self::PENDING_CANCEL),
            '7' => Ok(Self::STOPPED),
            '8' => Ok(Self::REJECTED),
            '9' => Ok(Self::SUSPENDED),
            'A' => Ok(Self::PENDING_NEW),
            'B' => Ok(Self::CALCULATED),
            'C' => Ok(Self::EXPIRED),
            'D' => Ok(Self::ACCEPTED_FOR_BIDDING),
            'E' => Ok(Self::PENDING_REPLACE),
            _ => Err(DecodeError::UnknownChar(Tags::OrdStatus, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum OrdType {
    MARKET = '1' as isize,
    LIMIT = '2' as isize,
    STOP = '3' as isize,
    STOP_LIMIT = '4' as isize,
    MARKET_ON_CLOSE = '5' as isize,
    WITH_OR_WITHOUT = '6' as isize,
    LIMIT_OR_BETTER = '7' as isize,
    LIMIT_WITH_OR_WITHOUT = '8' as isize,
    ON_BASIS = '9' as isize,
    ON_CLOSE = 'A' as isize,
    LIMIT_ON_CLOSE = 'B' as isize,
    FOREX_C = 'C' as isize,
    PREVIOUSLY_QUOTED = 'D' as isize,
    PREVIOUSLY_INDICATED = 'E' as isize,
    FOREX_F = 'F' as isize,
    FOREX_G = 'G' as isize,
    FOREX_H = 'H' as isize,
    FUNARI = 'I' as isize,
    PEGGED = 'P' as isize,
}

impl From<OrdType> for char {
    fn from(a: OrdType) -> char {
        a as isize as u8 as char
    }
}

impl From<OrdType> for &'static [u8] {
    fn from(a: OrdType) -> &'static [u8] {
        match a {
            OrdType::MARKET => b"1",
            OrdType::LIMIT => b"2",
            OrdType::STOP => b"3",
            OrdType::STOP_LIMIT => b"4",
            OrdType::MARKET_ON_CLOSE => b"5",
            OrdType::WITH_OR_WITHOUT => b"6",
            OrdType::LIMIT_OR_BETTER => b"7",
            OrdType::LIMIT_WITH_OR_WITHOUT => b"8",
            OrdType::ON_BASIS => b"9",
            OrdType::ON_CLOSE => b"A",
            OrdType::LIMIT_ON_CLOSE => b"B",
            OrdType::FOREX_C => b"C",
            OrdType::PREVIOUSLY_QUOTED => b"D",
            OrdType::PREVIOUSLY_INDICATED => b"E",
            OrdType::FOREX_F => b"F",
            OrdType::FOREX_G => b"G",
            OrdType::FOREX_H => b"H",
            OrdType::FUNARI => b"I",
            OrdType::PEGGED => b"P",
        }
    }
}

impl TryFrom<char> for OrdType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Self::MARKET),
            '2' => Ok(Self::LIMIT),
            '3' => Ok(Self::STOP),
            '4' => Ok(Self::STOP_LIMIT),
            '5' => Ok(Self::MARKET_ON_CLOSE),
            '6' => Ok(Self::WITH_OR_WITHOUT),
            '7' => Ok(Self::LIMIT_OR_BETTER),
            '8' => Ok(Self::LIMIT_WITH_OR_WITHOUT),
            '9' => Ok(Self::ON_BASIS),
            'A' => Ok(Self::ON_CLOSE),
            'B' => Ok(Self::LIMIT_ON_CLOSE),
            'C' => Ok(Self::FOREX_C),
            'D' => Ok(Self::PREVIOUSLY_QUOTED),
            'E' => Ok(Self::PREVIOUSLY_INDICATED),
            'F' => Ok(Self::FOREX_F),
            'G' => Ok(Self::FOREX_G),
            'H' => Ok(Self::FOREX_H),
            'I' => Ok(Self::FUNARI),
            'P' => Ok(Self::PEGGED),
            _ => Err(DecodeError::UnknownChar(Tags::OrdType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum PossDupFlag {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<PossDupFlag> for char {
    fn from(a: PossDupFlag) -> char {
        a as isize as u8 as char
    }
}

impl From<PossDupFlag> for &'static [u8] {
    fn from(a: PossDupFlag) -> &'static [u8] {
        match a {
            PossDupFlag::NO => b"N",
            PossDupFlag::YES => b"Y",
        }
    }
}

impl TryFrom<char> for PossDupFlag {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::PossDupFlag, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum Rule80A {
    AGENCY_SINGLE_ORDER = 'A' as isize,
    SHORT_EXEMPT_TRANSACTION_B = 'B' as isize,
    PROGRAM_ORDER_NON_INDEX_ARB_FOR_MEMBER_FIRM_ORG = 'C' as isize,
    PROGRAM_ORDER_INDEX_ARB_FOR_MEMBER_FIRM_ORG = 'D' as isize,
    REGISTERED_EQUITY_MARKET_MAKER_TRADES = 'E' as isize,
    SHORT_EXEMPT_TRANSACTION_F = 'F' as isize,
    SHORT_EXEMPT_TRANSACTION_H = 'H' as isize,
    INDIVIDUAL_INVESTOR_SINGLE_ORDER = 'I' as isize,
    PROGRAM_ORDER_INDEX_ARB_FOR_INDIVIDUAL_CUSTOMER = 'J' as isize,
    PROGRAM_ORDER_NON_INDEX_ARB_FOR_INDIVIDUAL_CUSTOMER = 'K' as isize,
    SHORT_EXEMPT_TRANSACTION_FOR_MEMBER_COMPETING_MARKET_MAKER_AFFILIATED_WITH_THE_FIRM_CLEARING_THE_TRADE =
        'L' as isize,
    PROGRAM_ORDER_INDEX_ARB_FOR_OTHER_MEMBER = 'M' as isize,
    PROGRAM_ORDER_NON_INDEX_ARB_FOR_OTHER_MEMBER = 'N' as isize,
    COMPETING_DEALER_TRADES_O = 'O' as isize,
    PRINCIPAL = 'P' as isize,
    COMPETING_DEALER_TRADES_R = 'R' as isize,
    SPECIALIST_TRADES = 'S' as isize,
    COMPETING_DEALER_TRADES_T = 'T' as isize,
    PROGRAM_ORDER_INDEX_ARB_FOR_OTHER_AGENCY = 'U' as isize,
    ALL_OTHER_ORDERS_AS_AGENT_FOR_OTHER_MEMBER = 'W' as isize,
    SHORT_EXEMPT_TRANSACTION_FOR_MEMBER_COMPETING_MARKET_MAKER_NOT_AFFILIATED_WITH_THE_FIRM_CLEARING_THE_TRADE =
        'X' as isize,
    PROGRAM_ORDER_NON_INDEX_ARB_FOR_OTHER_AGENCY = 'Y' as isize,
    SHORT_EXEMPT_TRANSACTION_FOR_NON_MEMBER_COMPETING_MARKET_MAKER = 'Z' as isize,
}

impl From<Rule80A> for char {
    fn from(a: Rule80A) -> char {
        a as isize as u8 as char
    }
}

impl From<Rule80A> for &'static [u8] {
    fn from(a: Rule80A) -> &'static [u8] {
        match a {
    Rule80A::AGENCY_SINGLE_ORDER => b"A",
    Rule80A::SHORT_EXEMPT_TRANSACTION_B => b"B",
    Rule80A::PROGRAM_ORDER_NON_INDEX_ARB_FOR_MEMBER_FIRM_ORG => b"C",
    Rule80A::PROGRAM_ORDER_INDEX_ARB_FOR_MEMBER_FIRM_ORG => b"D",
    Rule80A::REGISTERED_EQUITY_MARKET_MAKER_TRADES => b"E",
    Rule80A::SHORT_EXEMPT_TRANSACTION_F => b"F",
    Rule80A::SHORT_EXEMPT_TRANSACTION_H => b"H",
    Rule80A::INDIVIDUAL_INVESTOR_SINGLE_ORDER => b"I",
    Rule80A::PROGRAM_ORDER_INDEX_ARB_FOR_INDIVIDUAL_CUSTOMER => b"J",
    Rule80A::PROGRAM_ORDER_NON_INDEX_ARB_FOR_INDIVIDUAL_CUSTOMER => b"K",
    Rule80A::SHORT_EXEMPT_TRANSACTION_FOR_MEMBER_COMPETING_MARKET_MAKER_AFFILIATED_WITH_THE_FIRM_CLEARING_THE_TRADE => b"L",
    Rule80A::PROGRAM_ORDER_INDEX_ARB_FOR_OTHER_MEMBER => b"M",
    Rule80A::PROGRAM_ORDER_NON_INDEX_ARB_FOR_OTHER_MEMBER => b"N",
    Rule80A::COMPETING_DEALER_TRADES_O => b"O",
    Rule80A::PRINCIPAL => b"P",
    Rule80A::COMPETING_DEALER_TRADES_R => b"R",
    Rule80A::SPECIALIST_TRADES => b"S",
    Rule80A::COMPETING_DEALER_TRADES_T => b"T",
    Rule80A::PROGRAM_ORDER_INDEX_ARB_FOR_OTHER_AGENCY => b"U",
    Rule80A::ALL_OTHER_ORDERS_AS_AGENT_FOR_OTHER_MEMBER => b"W",
    Rule80A::SHORT_EXEMPT_TRANSACTION_FOR_MEMBER_COMPETING_MARKET_MAKER_NOT_AFFILIATED_WITH_THE_FIRM_CLEARING_THE_TRADE => b"X",
    Rule80A::PROGRAM_ORDER_NON_INDEX_ARB_FOR_OTHER_AGENCY => b"Y",
    Rule80A::SHORT_EXEMPT_TRANSACTION_FOR_NON_MEMBER_COMPETING_MARKET_MAKER => b"Z",
    
        }
    }
}

impl TryFrom<char> for Rule80A {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
    'A' => Ok(Self::AGENCY_SINGLE_ORDER),'B' => Ok(Self::SHORT_EXEMPT_TRANSACTION_B),'C' => Ok(Self::PROGRAM_ORDER_NON_INDEX_ARB_FOR_MEMBER_FIRM_ORG),'D' => Ok(Self::PROGRAM_ORDER_INDEX_ARB_FOR_MEMBER_FIRM_ORG),'E' => Ok(Self::REGISTERED_EQUITY_MARKET_MAKER_TRADES),'F' => Ok(Self::SHORT_EXEMPT_TRANSACTION_F),'H' => Ok(Self::SHORT_EXEMPT_TRANSACTION_H),'I' => Ok(Self::INDIVIDUAL_INVESTOR_SINGLE_ORDER),'J' => Ok(Self::PROGRAM_ORDER_INDEX_ARB_FOR_INDIVIDUAL_CUSTOMER),'K' => Ok(Self::PROGRAM_ORDER_NON_INDEX_ARB_FOR_INDIVIDUAL_CUSTOMER),'L' => Ok(Self::SHORT_EXEMPT_TRANSACTION_FOR_MEMBER_COMPETING_MARKET_MAKER_AFFILIATED_WITH_THE_FIRM_CLEARING_THE_TRADE),'M' => Ok(Self::PROGRAM_ORDER_INDEX_ARB_FOR_OTHER_MEMBER),'N' => Ok(Self::PROGRAM_ORDER_NON_INDEX_ARB_FOR_OTHER_MEMBER),'O' => Ok(Self::COMPETING_DEALER_TRADES_O),'P' => Ok(Self::PRINCIPAL),'R' => Ok(Self::COMPETING_DEALER_TRADES_R),'S' => Ok(Self::SPECIALIST_TRADES),'T' => Ok(Self::COMPETING_DEALER_TRADES_T),'U' => Ok(Self::PROGRAM_ORDER_INDEX_ARB_FOR_OTHER_AGENCY),'W' => Ok(Self::ALL_OTHER_ORDERS_AS_AGENT_FOR_OTHER_MEMBER),'X' => Ok(Self::SHORT_EXEMPT_TRANSACTION_FOR_MEMBER_COMPETING_MARKET_MAKER_NOT_AFFILIATED_WITH_THE_FIRM_CLEARING_THE_TRADE),'Y' => Ok(Self::PROGRAM_ORDER_NON_INDEX_ARB_FOR_OTHER_AGENCY),'Z' => Ok(Self::SHORT_EXEMPT_TRANSACTION_FOR_NON_MEMBER_COMPETING_MARKET_MAKER),
    _=> Err(DecodeError::UnknownChar(Tags::Rule80A, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum Side {
    BUY = '1' as isize,
    SELL = '2' as isize,
    BUY_MINUS = '3' as isize,
    SELL_PLUS = '4' as isize,
    SELL_SHORT = '5' as isize,
    SELL_SHORT_EXEMPT = '6' as isize,
    UNDISCLOSED = '7' as isize,
    CROSS = '8' as isize,
    CROSS_SHORT = '9' as isize,
}

impl From<Side> for char {
    fn from(a: Side) -> char {
        a as isize as u8 as char
    }
}

impl From<Side> for &'static [u8] {
    fn from(a: Side) -> &'static [u8] {
        match a {
            Side::BUY => b"1",
            Side::SELL => b"2",
            Side::BUY_MINUS => b"3",
            Side::SELL_PLUS => b"4",
            Side::SELL_SHORT => b"5",
            Side::SELL_SHORT_EXEMPT => b"6",
            Side::UNDISCLOSED => b"7",
            Side::CROSS => b"8",
            Side::CROSS_SHORT => b"9",
        }
    }
}

impl TryFrom<char> for Side {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Self::BUY),
            '2' => Ok(Self::SELL),
            '3' => Ok(Self::BUY_MINUS),
            '4' => Ok(Self::SELL_PLUS),
            '5' => Ok(Self::SELL_SHORT),
            '6' => Ok(Self::SELL_SHORT_EXEMPT),
            '7' => Ok(Self::UNDISCLOSED),
            '8' => Ok(Self::CROSS),
            '9' => Ok(Self::CROSS_SHORT),
            _ => Err(DecodeError::UnknownChar(Tags::Side, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum TimeInForce {
    DAY = '0' as isize,
    GOOD_TILL_CANCEL = '1' as isize,
    AT_THE_OPENING = '2' as isize,
    IMMEDIATE_OR_CANCEL = '3' as isize,
    FILL_OR_KILL = '4' as isize,
    GOOD_TILL_CROSSING = '5' as isize,
    GOOD_TILL_DATE = '6' as isize,
}

impl From<TimeInForce> for char {
    fn from(a: TimeInForce) -> char {
        a as isize as u8 as char
    }
}

impl From<TimeInForce> for &'static [u8] {
    fn from(a: TimeInForce) -> &'static [u8] {
        match a {
            TimeInForce::DAY => b"0",
            TimeInForce::GOOD_TILL_CANCEL => b"1",
            TimeInForce::AT_THE_OPENING => b"2",
            TimeInForce::IMMEDIATE_OR_CANCEL => b"3",
            TimeInForce::FILL_OR_KILL => b"4",
            TimeInForce::GOOD_TILL_CROSSING => b"5",
            TimeInForce::GOOD_TILL_DATE => b"6",
        }
    }
}

impl TryFrom<char> for TimeInForce {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::DAY),
            '1' => Ok(Self::GOOD_TILL_CANCEL),
            '2' => Ok(Self::AT_THE_OPENING),
            '3' => Ok(Self::IMMEDIATE_OR_CANCEL),
            '4' => Ok(Self::FILL_OR_KILL),
            '5' => Ok(Self::GOOD_TILL_CROSSING),
            '6' => Ok(Self::GOOD_TILL_DATE),
            _ => Err(DecodeError::UnknownChar(Tags::TimeInForce, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum Urgency {
    NORMAL = '0' as isize,
    FLASH = '1' as isize,
    BACKGROUND = '2' as isize,
}

impl From<Urgency> for char {
    fn from(a: Urgency) -> char {
        a as isize as u8 as char
    }
}

impl From<Urgency> for &'static [u8] {
    fn from(a: Urgency) -> &'static [u8] {
        match a {
            Urgency::NORMAL => b"0",
            Urgency::FLASH => b"1",
            Urgency::BACKGROUND => b"2",
        }
    }
}

impl TryFrom<char> for Urgency {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::NORMAL),
            '1' => Ok(Self::FLASH),
            '2' => Ok(Self::BACKGROUND),
            _ => Err(DecodeError::UnknownChar(Tags::Urgency, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum SettlmntTyp {
    REGULAR = '0' as isize,
    CASH = '1' as isize,
    NEXT_DAY = '2' as isize,
    T_PLUS_2 = '3' as isize,
    T_PLUS_3 = '4' as isize,
    T_PLUS_4 = '5' as isize,
    FUTURE = '6' as isize,
    WHEN_ISSUED = '7' as isize,
    SELLERS_OPTION = '8' as isize,
    T_PLUS_5 = '9' as isize,
}

impl From<SettlmntTyp> for char {
    fn from(a: SettlmntTyp) -> char {
        a as isize as u8 as char
    }
}

impl From<SettlmntTyp> for &'static [u8] {
    fn from(a: SettlmntTyp) -> &'static [u8] {
        match a {
            SettlmntTyp::REGULAR => b"0",
            SettlmntTyp::CASH => b"1",
            SettlmntTyp::NEXT_DAY => b"2",
            SettlmntTyp::T_PLUS_2 => b"3",
            SettlmntTyp::T_PLUS_3 => b"4",
            SettlmntTyp::T_PLUS_4 => b"5",
            SettlmntTyp::FUTURE => b"6",
            SettlmntTyp::WHEN_ISSUED => b"7",
            SettlmntTyp::SELLERS_OPTION => b"8",
            SettlmntTyp::T_PLUS_5 => b"9",
        }
    }
}

impl TryFrom<char> for SettlmntTyp {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::REGULAR),
            '1' => Ok(Self::CASH),
            '2' => Ok(Self::NEXT_DAY),
            '3' => Ok(Self::T_PLUS_2),
            '4' => Ok(Self::T_PLUS_3),
            '5' => Ok(Self::T_PLUS_4),
            '6' => Ok(Self::FUTURE),
            '7' => Ok(Self::WHEN_ISSUED),
            '8' => Ok(Self::SELLERS_OPTION),
            '9' => Ok(Self::T_PLUS_5),
            _ => Err(DecodeError::UnknownChar(Tags::SettlmntTyp, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum AllocTransType {
    NEW = '0' as isize,
    REPLACE = '1' as isize,
    CANCEL = '2' as isize,
    PRELIMINARY = '3' as isize,
    CALCULATED = '4' as isize,
    CALCULATED_WITHOUT_PRELIMINARY = '5' as isize,
}

impl From<AllocTransType> for char {
    fn from(a: AllocTransType) -> char {
        a as isize as u8 as char
    }
}

impl From<AllocTransType> for &'static [u8] {
    fn from(a: AllocTransType) -> &'static [u8] {
        match a {
            AllocTransType::NEW => b"0",
            AllocTransType::REPLACE => b"1",
            AllocTransType::CANCEL => b"2",
            AllocTransType::PRELIMINARY => b"3",
            AllocTransType::CALCULATED => b"4",
            AllocTransType::CALCULATED_WITHOUT_PRELIMINARY => b"5",
        }
    }
}

impl TryFrom<char> for AllocTransType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::NEW),
            '1' => Ok(Self::REPLACE),
            '2' => Ok(Self::CANCEL),
            '3' => Ok(Self::PRELIMINARY),
            '4' => Ok(Self::CALCULATED),
            '5' => Ok(Self::CALCULATED_WITHOUT_PRELIMINARY),
            _ => Err(DecodeError::UnknownChar(Tags::AllocTransType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum OpenClose {
    CLOSE = 'C' as isize,
    OPEN = 'O' as isize,
}

impl From<OpenClose> for char {
    fn from(a: OpenClose) -> char {
        a as isize as u8 as char
    }
}

impl From<OpenClose> for &'static [u8] {
    fn from(a: OpenClose) -> &'static [u8] {
        match a {
            OpenClose::CLOSE => b"C",
            OpenClose::OPEN => b"O",
        }
    }
}

impl TryFrom<char> for OpenClose {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'C' => Ok(Self::CLOSE),
            'O' => Ok(Self::OPEN),
            _ => Err(DecodeError::UnknownChar(Tags::OpenClose, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum ProcessCode {
    REGULAR = '0' as isize,
    SOFT_DOLLAR = '1' as isize,
    STEP_IN = '2' as isize,
    STEP_OUT = '3' as isize,
    SOFT_DOLLAR_STEP_IN = '4' as isize,
    SOFT_DOLLAR_STEP_OUT = '5' as isize,
    PLAN_SPONSOR = '6' as isize,
}

impl From<ProcessCode> for char {
    fn from(a: ProcessCode) -> char {
        a as isize as u8 as char
    }
}

impl From<ProcessCode> for &'static [u8] {
    fn from(a: ProcessCode) -> &'static [u8] {
        match a {
            ProcessCode::REGULAR => b"0",
            ProcessCode::SOFT_DOLLAR => b"1",
            ProcessCode::STEP_IN => b"2",
            ProcessCode::STEP_OUT => b"3",
            ProcessCode::SOFT_DOLLAR_STEP_IN => b"4",
            ProcessCode::SOFT_DOLLAR_STEP_OUT => b"5",
            ProcessCode::PLAN_SPONSOR => b"6",
        }
    }
}

impl TryFrom<char> for ProcessCode {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::REGULAR),
            '1' => Ok(Self::SOFT_DOLLAR),
            '2' => Ok(Self::STEP_IN),
            '3' => Ok(Self::STEP_OUT),
            '4' => Ok(Self::SOFT_DOLLAR_STEP_IN),
            '5' => Ok(Self::SOFT_DOLLAR_STEP_OUT),
            '6' => Ok(Self::PLAN_SPONSOR),
            _ => Err(DecodeError::UnknownChar(Tags::ProcessCode, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum AllocStatus {
    ACCEPTED = 0,
    REJECTED = 1,
    PARTIAL_ACCEPT = 2,
    RECEIVED = 3,
}

impl TryFrom<u8> for AllocStatus {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::ACCEPTED),
            1 => Ok(Self::REJECTED),
            2 => Ok(Self::PARTIAL_ACCEPT),
            3 => Ok(Self::RECEIVED),
            _ => Err(DecodeError::UnknownInt(Tags::AllocStatus, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum AllocRejCode {
    UNKNOWN_ACCOUNT = 0,
    INCORRECT_QUANTITY = 1,
    INCORRECT_AVERAGE_PRICE = 2,
    UNKNOWN_EXECUTING_BROKER_MNEMONIC = 3,
    COMMISSION_DIFFERENCE = 4,
    UNKNOWN_ORDERID = 5,
    UNKNOWN_LISTID = 6,
    OTHER = 7,
}

impl TryFrom<u8> for AllocRejCode {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::UNKNOWN_ACCOUNT),
            1 => Ok(Self::INCORRECT_QUANTITY),
            2 => Ok(Self::INCORRECT_AVERAGE_PRICE),
            3 => Ok(Self::UNKNOWN_EXECUTING_BROKER_MNEMONIC),
            4 => Ok(Self::COMMISSION_DIFFERENCE),
            5 => Ok(Self::UNKNOWN_ORDERID),
            6 => Ok(Self::UNKNOWN_LISTID),
            7 => Ok(Self::OTHER),
            _ => Err(DecodeError::UnknownInt(Tags::AllocRejCode, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum EmailType {
    NEW = '0' as isize,
    REPLY = '1' as isize,
    ADMIN_REPLY = '2' as isize,
}

impl From<EmailType> for char {
    fn from(a: EmailType) -> char {
        a as isize as u8 as char
    }
}

impl From<EmailType> for &'static [u8] {
    fn from(a: EmailType) -> &'static [u8] {
        match a {
            EmailType::NEW => b"0",
            EmailType::REPLY => b"1",
            EmailType::ADMIN_REPLY => b"2",
        }
    }
}

impl TryFrom<char> for EmailType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::NEW),
            '1' => Ok(Self::REPLY),
            '2' => Ok(Self::ADMIN_REPLY),
            _ => Err(DecodeError::UnknownChar(Tags::EmailType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum PossResend {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<PossResend> for char {
    fn from(a: PossResend) -> char {
        a as isize as u8 as char
    }
}

impl From<PossResend> for &'static [u8] {
    fn from(a: PossResend) -> &'static [u8] {
        match a {
            PossResend::NO => b"N",
            PossResend::YES => b"Y",
        }
    }
}

impl TryFrom<char> for PossResend {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::PossResend, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum EncryptMethod {
    NONE = 0,
    PKCS = 1,
    DES = 2,
    PKCS_DES = 3,
    PGP_DES = 4,
    PGP_DES_MD5 = 5,
    PEM_DES_MD5 = 6,
}

impl TryFrom<u8> for EncryptMethod {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::NONE),
            1 => Ok(Self::PKCS),
            2 => Ok(Self::DES),
            3 => Ok(Self::PKCS_DES),
            4 => Ok(Self::PGP_DES),
            5 => Ok(Self::PGP_DES_MD5),
            6 => Ok(Self::PEM_DES_MD5),
            _ => Err(DecodeError::UnknownInt(Tags::EncryptMethod, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum CxlRejReason {
    TOO_LATE_TO_CANCEL = 0,
    UNKNOWN_ORDER = 1,
    BROKER_OPTION = 2,
    ORDER_ALREADY_IN_PENDING_CANCEL_OR_PENDING_REPLACE_STATUS = 3,
}

impl TryFrom<u8> for CxlRejReason {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::TOO_LATE_TO_CANCEL),
            1 => Ok(Self::UNKNOWN_ORDER),
            2 => Ok(Self::BROKER_OPTION),
            3 => Ok(Self::ORDER_ALREADY_IN_PENDING_CANCEL_OR_PENDING_REPLACE_STATUS),
            _ => Err(DecodeError::UnknownInt(Tags::CxlRejReason, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum OrdRejReason {
    BROKER_OPTION = 0,
    UNKNOWN_SYMBOL = 1,
    EXCHANGE_CLOSED = 2,
    ORDER_EXCEEDS_LIMIT = 3,
    TOO_LATE_TO_ENTER = 4,
    UNKNOWN_ORDER = 5,
    DUPLICATE_ORDER = 6,
    DUPLICATE_OF_A_VERBALLY_COMMUNICATED_ORDER = 7,
    STALE_ORDER = 8,
}

impl TryFrom<u8> for OrdRejReason {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::BROKER_OPTION),
            1 => Ok(Self::UNKNOWN_SYMBOL),
            2 => Ok(Self::EXCHANGE_CLOSED),
            3 => Ok(Self::ORDER_EXCEEDS_LIMIT),
            4 => Ok(Self::TOO_LATE_TO_ENTER),
            5 => Ok(Self::UNKNOWN_ORDER),
            6 => Ok(Self::DUPLICATE_ORDER),
            7 => Ok(Self::DUPLICATE_OF_A_VERBALLY_COMMUNICATED_ORDER),
            8 => Ok(Self::STALE_ORDER),
            _ => Err(DecodeError::UnknownInt(Tags::OrdRejReason, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum IOIQualifier {
    ALL_OR_NONE = 'A' as isize,
    AT_THE_CLOSE = 'C' as isize,
    IN_TOUCH_WITH = 'I' as isize,
    LIMIT = 'L' as isize,
    MORE_BEHIND = 'M' as isize,
    AT_THE_OPEN = 'O' as isize,
    TAKING_A_POSITION = 'P' as isize,
    AT_THE_MARKET = 'Q' as isize,
    READY_TO_TRADE = 'R' as isize,
    PORTFOLIO_SHOW_N = 'S' as isize,
    THROUGH_THE_DAY = 'T' as isize,
    VERSUS = 'V' as isize,
    INDICATION = 'W' as isize,
    CROSSING_OPPORTUNITY = 'X' as isize,
    AT_THE_MIDPOINT = 'Y' as isize,
    PRE_OPEN = 'Z' as isize,
}

impl From<IOIQualifier> for char {
    fn from(a: IOIQualifier) -> char {
        a as isize as u8 as char
    }
}

impl From<IOIQualifier> for &'static [u8] {
    fn from(a: IOIQualifier) -> &'static [u8] {
        match a {
            IOIQualifier::ALL_OR_NONE => b"A",
            IOIQualifier::AT_THE_CLOSE => b"C",
            IOIQualifier::IN_TOUCH_WITH => b"I",
            IOIQualifier::LIMIT => b"L",
            IOIQualifier::MORE_BEHIND => b"M",
            IOIQualifier::AT_THE_OPEN => b"O",
            IOIQualifier::TAKING_A_POSITION => b"P",
            IOIQualifier::AT_THE_MARKET => b"Q",
            IOIQualifier::READY_TO_TRADE => b"R",
            IOIQualifier::PORTFOLIO_SHOW_N => b"S",
            IOIQualifier::THROUGH_THE_DAY => b"T",
            IOIQualifier::VERSUS => b"V",
            IOIQualifier::INDICATION => b"W",
            IOIQualifier::CROSSING_OPPORTUNITY => b"X",
            IOIQualifier::AT_THE_MIDPOINT => b"Y",
            IOIQualifier::PRE_OPEN => b"Z",
        }
    }
}

impl TryFrom<char> for IOIQualifier {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Self::ALL_OR_NONE),
            'C' => Ok(Self::AT_THE_CLOSE),
            'I' => Ok(Self::IN_TOUCH_WITH),
            'L' => Ok(Self::LIMIT),
            'M' => Ok(Self::MORE_BEHIND),
            'O' => Ok(Self::AT_THE_OPEN),
            'P' => Ok(Self::TAKING_A_POSITION),
            'Q' => Ok(Self::AT_THE_MARKET),
            'R' => Ok(Self::READY_TO_TRADE),
            'S' => Ok(Self::PORTFOLIO_SHOW_N),
            'T' => Ok(Self::THROUGH_THE_DAY),
            'V' => Ok(Self::VERSUS),
            'W' => Ok(Self::INDICATION),
            'X' => Ok(Self::CROSSING_OPPORTUNITY),
            'Y' => Ok(Self::AT_THE_MIDPOINT),
            'Z' => Ok(Self::PRE_OPEN),
            _ => Err(DecodeError::UnknownChar(Tags::IOIQualifier, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum ReportToExch {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<ReportToExch> for char {
    fn from(a: ReportToExch) -> char {
        a as isize as u8 as char
    }
}

impl From<ReportToExch> for &'static [u8] {
    fn from(a: ReportToExch) -> &'static [u8] {
        match a {
            ReportToExch::NO => b"N",
            ReportToExch::YES => b"Y",
        }
    }
}

impl TryFrom<char> for ReportToExch {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::ReportToExch, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum LocateReqd {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<LocateReqd> for char {
    fn from(a: LocateReqd) -> char {
        a as isize as u8 as char
    }
}

impl From<LocateReqd> for &'static [u8] {
    fn from(a: LocateReqd) -> &'static [u8] {
        match a {
            LocateReqd::NO => b"N",
            LocateReqd::YES => b"Y",
        }
    }
}

impl TryFrom<char> for LocateReqd {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::LocateReqd, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum ForexReq {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<ForexReq> for char {
    fn from(a: ForexReq) -> char {
        a as isize as u8 as char
    }
}

impl From<ForexReq> for &'static [u8] {
    fn from(a: ForexReq) -> &'static [u8] {
        match a {
            ForexReq::NO => b"N",
            ForexReq::YES => b"Y",
        }
    }
}

impl TryFrom<char> for ForexReq {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::ForexReq, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum GapFillFlag {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<GapFillFlag> for char {
    fn from(a: GapFillFlag) -> char {
        a as isize as u8 as char
    }
}

impl From<GapFillFlag> for &'static [u8] {
    fn from(a: GapFillFlag) -> &'static [u8] {
        match a {
            GapFillFlag::NO => b"N",
            GapFillFlag::YES => b"Y",
        }
    }
}

impl TryFrom<char> for GapFillFlag {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::GapFillFlag, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum DKReason {
    UNKNOWN_SYMBOL = 'A' as isize,
    WRONG_SIDE = 'B' as isize,
    QUANTITY_EXCEEDS_ORDER = 'C' as isize,
    NO_MATCHING_ORDER = 'D' as isize,
    PRICE_EXCEEDS_LIMIT = 'E' as isize,
    OTHER = 'Z' as isize,
}

impl From<DKReason> for char {
    fn from(a: DKReason) -> char {
        a as isize as u8 as char
    }
}

impl From<DKReason> for &'static [u8] {
    fn from(a: DKReason) -> &'static [u8] {
        match a {
            DKReason::UNKNOWN_SYMBOL => b"A",
            DKReason::WRONG_SIDE => b"B",
            DKReason::QUANTITY_EXCEEDS_ORDER => b"C",
            DKReason::NO_MATCHING_ORDER => b"D",
            DKReason::PRICE_EXCEEDS_LIMIT => b"E",
            DKReason::OTHER => b"Z",
        }
    }
}

impl TryFrom<char> for DKReason {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Self::UNKNOWN_SYMBOL),
            'B' => Ok(Self::WRONG_SIDE),
            'C' => Ok(Self::QUANTITY_EXCEEDS_ORDER),
            'D' => Ok(Self::NO_MATCHING_ORDER),
            'E' => Ok(Self::PRICE_EXCEEDS_LIMIT),
            'Z' => Ok(Self::OTHER),
            _ => Err(DecodeError::UnknownChar(Tags::DKReason, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum IOINaturalFlag {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<IOINaturalFlag> for char {
    fn from(a: IOINaturalFlag) -> char {
        a as isize as u8 as char
    }
}

impl From<IOINaturalFlag> for &'static [u8] {
    fn from(a: IOINaturalFlag) -> &'static [u8] {
        match a {
            IOINaturalFlag::NO => b"N",
            IOINaturalFlag::YES => b"Y",
        }
    }
}

impl TryFrom<char> for IOINaturalFlag {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::IOINaturalFlag, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum MiscFeeType {
    REGULATORY = '1' as isize,
    TAX = '2' as isize,
    LOCAL_COMMISSION = '3' as isize,
    EXCHANGE_FEES = '4' as isize,
    STAMP = '5' as isize,
    LEVY = '6' as isize,
    OTHER = '7' as isize,
    MARKUP = '8' as isize,
    CONSUMPTION_TAX = '9' as isize,
}

impl From<MiscFeeType> for char {
    fn from(a: MiscFeeType) -> char {
        a as isize as u8 as char
    }
}

impl From<MiscFeeType> for &'static [u8] {
    fn from(a: MiscFeeType) -> &'static [u8] {
        match a {
            MiscFeeType::REGULATORY => b"1",
            MiscFeeType::TAX => b"2",
            MiscFeeType::LOCAL_COMMISSION => b"3",
            MiscFeeType::EXCHANGE_FEES => b"4",
            MiscFeeType::STAMP => b"5",
            MiscFeeType::LEVY => b"6",
            MiscFeeType::OTHER => b"7",
            MiscFeeType::MARKUP => b"8",
            MiscFeeType::CONSUMPTION_TAX => b"9",
        }
    }
}

impl TryFrom<char> for MiscFeeType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Self::REGULATORY),
            '2' => Ok(Self::TAX),
            '3' => Ok(Self::LOCAL_COMMISSION),
            '4' => Ok(Self::EXCHANGE_FEES),
            '5' => Ok(Self::STAMP),
            '6' => Ok(Self::LEVY),
            '7' => Ok(Self::OTHER),
            '8' => Ok(Self::MARKUP),
            '9' => Ok(Self::CONSUMPTION_TAX),
            _ => Err(DecodeError::UnknownChar(Tags::MiscFeeType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum ResetSeqNumFlag {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<ResetSeqNumFlag> for char {
    fn from(a: ResetSeqNumFlag) -> char {
        a as isize as u8 as char
    }
}

impl From<ResetSeqNumFlag> for &'static [u8] {
    fn from(a: ResetSeqNumFlag) -> &'static [u8] {
        match a {
            ResetSeqNumFlag::NO => b"N",
            ResetSeqNumFlag::YES => b"Y",
        }
    }
}

impl TryFrom<char> for ResetSeqNumFlag {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::ResetSeqNumFlag, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum ExecType {
    NEW = '0' as isize,
    PARTIAL_FILL = '1' as isize,
    FILL = '2' as isize,
    DONE_FOR_DAY = '3' as isize,
    CANCELED = '4' as isize,
    REPLACE = '5' as isize,
    PENDING_CANCEL = '6' as isize,
    STOPPED = '7' as isize,
    REJECTED = '8' as isize,
    SUSPENDED = '9' as isize,
    PENDING_NEW = 'A' as isize,
    CALCULATED = 'B' as isize,
    EXPIRED = 'C' as isize,
    RESTATED = 'D' as isize,
    PENDING_REPLACE = 'E' as isize,
}

impl From<ExecType> for char {
    fn from(a: ExecType) -> char {
        a as isize as u8 as char
    }
}

impl From<ExecType> for &'static [u8] {
    fn from(a: ExecType) -> &'static [u8] {
        match a {
            ExecType::NEW => b"0",
            ExecType::PARTIAL_FILL => b"1",
            ExecType::FILL => b"2",
            ExecType::DONE_FOR_DAY => b"3",
            ExecType::CANCELED => b"4",
            ExecType::REPLACE => b"5",
            ExecType::PENDING_CANCEL => b"6",
            ExecType::STOPPED => b"7",
            ExecType::REJECTED => b"8",
            ExecType::SUSPENDED => b"9",
            ExecType::PENDING_NEW => b"A",
            ExecType::CALCULATED => b"B",
            ExecType::EXPIRED => b"C",
            ExecType::RESTATED => b"D",
            ExecType::PENDING_REPLACE => b"E",
        }
    }
}

impl TryFrom<char> for ExecType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::NEW),
            '1' => Ok(Self::PARTIAL_FILL),
            '2' => Ok(Self::FILL),
            '3' => Ok(Self::DONE_FOR_DAY),
            '4' => Ok(Self::CANCELED),
            '5' => Ok(Self::REPLACE),
            '6' => Ok(Self::PENDING_CANCEL),
            '7' => Ok(Self::STOPPED),
            '8' => Ok(Self::REJECTED),
            '9' => Ok(Self::SUSPENDED),
            'A' => Ok(Self::PENDING_NEW),
            'B' => Ok(Self::CALCULATED),
            'C' => Ok(Self::EXPIRED),
            'D' => Ok(Self::RESTATED),
            'E' => Ok(Self::PENDING_REPLACE),
            _ => Err(DecodeError::UnknownChar(Tags::ExecType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum SettlCurrFxRateCalc {
    MULTIPLY = 'M' as isize,
    DIVIDE = 'D' as isize,
}

impl From<SettlCurrFxRateCalc> for char {
    fn from(a: SettlCurrFxRateCalc) -> char {
        a as isize as u8 as char
    }
}

impl From<SettlCurrFxRateCalc> for &'static [u8] {
    fn from(a: SettlCurrFxRateCalc) -> &'static [u8] {
        match a {
            SettlCurrFxRateCalc::MULTIPLY => b"M",
            SettlCurrFxRateCalc::DIVIDE => b"D",
        }
    }
}

impl TryFrom<char> for SettlCurrFxRateCalc {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'M' => Ok(Self::MULTIPLY),
            'D' => Ok(Self::DIVIDE),
            _ => Err(DecodeError::UnknownChar(Tags::SettlCurrFxRateCalc, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum SettlInstMode {
    DEFAULT = '0' as isize,
    STANDING_INSTRUCTIONS_PROVIDED = '1' as isize,
    SPECIFIC_ALLOCATION_ACCOUNT_OVERRIDING = '2' as isize,
    SPECIFIC_ALLOCATION_ACCOUNT_STANDING = '3' as isize,
}

impl From<SettlInstMode> for char {
    fn from(a: SettlInstMode) -> char {
        a as isize as u8 as char
    }
}

impl From<SettlInstMode> for &'static [u8] {
    fn from(a: SettlInstMode) -> &'static [u8] {
        match a {
            SettlInstMode::DEFAULT => b"0",
            SettlInstMode::STANDING_INSTRUCTIONS_PROVIDED => b"1",
            SettlInstMode::SPECIFIC_ALLOCATION_ACCOUNT_OVERRIDING => b"2",
            SettlInstMode::SPECIFIC_ALLOCATION_ACCOUNT_STANDING => b"3",
        }
    }
}

impl TryFrom<char> for SettlInstMode {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::DEFAULT),
            '1' => Ok(Self::STANDING_INSTRUCTIONS_PROVIDED),
            '2' => Ok(Self::SPECIFIC_ALLOCATION_ACCOUNT_OVERRIDING),
            '3' => Ok(Self::SPECIFIC_ALLOCATION_ACCOUNT_STANDING),
            _ => Err(DecodeError::UnknownChar(Tags::SettlInstMode, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum SettlInstTransType {
    CANCEL = 'C' as isize,
    NEW = 'N' as isize,
    REPLACE = 'R' as isize,
}

impl From<SettlInstTransType> for char {
    fn from(a: SettlInstTransType) -> char {
        a as isize as u8 as char
    }
}

impl From<SettlInstTransType> for &'static [u8] {
    fn from(a: SettlInstTransType) -> &'static [u8] {
        match a {
            SettlInstTransType::CANCEL => b"C",
            SettlInstTransType::NEW => b"N",
            SettlInstTransType::REPLACE => b"R",
        }
    }
}

impl TryFrom<char> for SettlInstTransType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'C' => Ok(Self::CANCEL),
            'N' => Ok(Self::NEW),
            'R' => Ok(Self::REPLACE),
            _ => Err(DecodeError::UnknownChar(Tags::SettlInstTransType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum SettlInstSource {
    BROKERS_INSTRUCTIONS = '1' as isize,
    INSTITUTIONS_INSTRUCTIONS = '2' as isize,
}

impl From<SettlInstSource> for char {
    fn from(a: SettlInstSource) -> char {
        a as isize as u8 as char
    }
}

impl From<SettlInstSource> for &'static [u8] {
    fn from(a: SettlInstSource) -> &'static [u8] {
        match a {
            SettlInstSource::BROKERS_INSTRUCTIONS => b"1",
            SettlInstSource::INSTITUTIONS_INSTRUCTIONS => b"2",
        }
    }
}

impl TryFrom<char> for SettlInstSource {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Self::BROKERS_INSTRUCTIONS),
            '2' => Ok(Self::INSTITUTIONS_INSTRUCTIONS),
            _ => Err(DecodeError::UnknownChar(Tags::SettlInstSource, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum StandInstDbType {
    OTHER = 0,
    DTC_SID = 1,
    THOMSON_ALERT = 2,
    A_GLOBAL_CUSTODIAN = 3,
}

impl TryFrom<u8> for StandInstDbType {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::OTHER),
            1 => Ok(Self::DTC_SID),
            2 => Ok(Self::THOMSON_ALERT),
            3 => Ok(Self::A_GLOBAL_CUSTODIAN),
            _ => Err(DecodeError::UnknownInt(Tags::StandInstDbType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum AllocLinkType {
    F_X_NETTING = 0,
    F_X_SWAP = 1,
}

impl TryFrom<u8> for AllocLinkType {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::F_X_NETTING),
            1 => Ok(Self::F_X_SWAP),
            _ => Err(DecodeError::UnknownInt(Tags::AllocLinkType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum PutOrCall {
    PUT = 0,
    CALL = 1,
}

impl TryFrom<u8> for PutOrCall {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::PUT),
            1 => Ok(Self::CALL),
            _ => Err(DecodeError::UnknownInt(Tags::PutOrCall, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum CoveredOrUncovered {
    COVERED = 0,
    UNCOVERED = 1,
}

impl TryFrom<u8> for CoveredOrUncovered {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::COVERED),
            1 => Ok(Self::UNCOVERED),
            _ => Err(DecodeError::UnknownInt(Tags::CoveredOrUncovered, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum CustomerOrFirm {
    CUSTOMER = 0,
    FIRM = 1,
}

impl TryFrom<u8> for CustomerOrFirm {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::CUSTOMER),
            1 => Ok(Self::FIRM),
            _ => Err(DecodeError::UnknownInt(Tags::CustomerOrFirm, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum NotifyBrokerOfCredit {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<NotifyBrokerOfCredit> for char {
    fn from(a: NotifyBrokerOfCredit) -> char {
        a as isize as u8 as char
    }
}

impl From<NotifyBrokerOfCredit> for &'static [u8] {
    fn from(a: NotifyBrokerOfCredit) -> &'static [u8] {
        match a {
            NotifyBrokerOfCredit::NO => b"N",
            NotifyBrokerOfCredit::YES => b"Y",
        }
    }
}

impl TryFrom<char> for NotifyBrokerOfCredit {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::NotifyBrokerOfCredit, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum AllocHandlInst {
    MATCH = 1,
    FORWARD = 2,
    FORWARD_AND_MATCH = 3,
}

impl TryFrom<u8> for AllocHandlInst {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::MATCH),
            2 => Ok(Self::FORWARD),
            3 => Ok(Self::FORWARD_AND_MATCH),
            _ => Err(DecodeError::UnknownInt(Tags::AllocHandlInst, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum RoutingType {
    TARGET_FIRM = 1,
    TARGET_LIST = 2,
    BLOCK_FIRM = 3,
    BLOCK_LIST = 4,
}

impl TryFrom<u8> for RoutingType {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::TARGET_FIRM),
            2 => Ok(Self::TARGET_LIST),
            3 => Ok(Self::BLOCK_FIRM),
            4 => Ok(Self::BLOCK_LIST),
            _ => Err(DecodeError::UnknownInt(Tags::RoutingType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum Benchmark {
    CURVE = '1' as isize,
    FIVE_YR = '2' as isize,
    OLD_5 = '3' as isize,
    TEN_YR = '4' as isize,
    OLD_10 = '5' as isize,
    THIRTY_YR = '6' as isize,
    OLD_30 = '7' as isize,
    THREE_MO_LIBOR = '8' as isize,
    SIX_MO_LIBOR = '9' as isize,
}

impl From<Benchmark> for char {
    fn from(a: Benchmark) -> char {
        a as isize as u8 as char
    }
}

impl From<Benchmark> for &'static [u8] {
    fn from(a: Benchmark) -> &'static [u8] {
        match a {
            Benchmark::CURVE => b"1",
            Benchmark::FIVE_YR => b"2",
            Benchmark::OLD_5 => b"3",
            Benchmark::TEN_YR => b"4",
            Benchmark::OLD_10 => b"5",
            Benchmark::THIRTY_YR => b"6",
            Benchmark::OLD_30 => b"7",
            Benchmark::THREE_MO_LIBOR => b"8",
            Benchmark::SIX_MO_LIBOR => b"9",
        }
    }
}

impl TryFrom<char> for Benchmark {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Self::CURVE),
            '2' => Ok(Self::FIVE_YR),
            '3' => Ok(Self::OLD_5),
            '4' => Ok(Self::TEN_YR),
            '5' => Ok(Self::OLD_10),
            '6' => Ok(Self::THIRTY_YR),
            '7' => Ok(Self::OLD_30),
            '8' => Ok(Self::THREE_MO_LIBOR),
            '9' => Ok(Self::SIX_MO_LIBOR),
            _ => Err(DecodeError::UnknownChar(Tags::Benchmark, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum SubscriptionRequestType {
    SNAPSHOT = '0' as isize,
    SNAPSHOT_PLUS_UPDATES = '1' as isize,
    DISABLE_PREVIOUS_SNAPSHOT_PLUS_UPDATE_REQUEST = '2' as isize,
}

impl From<SubscriptionRequestType> for char {
    fn from(a: SubscriptionRequestType) -> char {
        a as isize as u8 as char
    }
}

impl From<SubscriptionRequestType> for &'static [u8] {
    fn from(a: SubscriptionRequestType) -> &'static [u8] {
        match a {
            SubscriptionRequestType::SNAPSHOT => b"0",
            SubscriptionRequestType::SNAPSHOT_PLUS_UPDATES => b"1",
            SubscriptionRequestType::DISABLE_PREVIOUS_SNAPSHOT_PLUS_UPDATE_REQUEST => b"2",
        }
    }
}

impl TryFrom<char> for SubscriptionRequestType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::SNAPSHOT),
            '1' => Ok(Self::SNAPSHOT_PLUS_UPDATES),
            '2' => Ok(Self::DISABLE_PREVIOUS_SNAPSHOT_PLUS_UPDATE_REQUEST),
            _ => Err(DecodeError::UnknownChar(Tags::SubscriptionRequestType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum MDUpdateType {
    FULL_REFRESH = 0,
    INCREMENTAL_REFRESH = 1,
}

impl TryFrom<u8> for MDUpdateType {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::FULL_REFRESH),
            1 => Ok(Self::INCREMENTAL_REFRESH),
            _ => Err(DecodeError::UnknownInt(Tags::MDUpdateType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum AggregatedBook {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<AggregatedBook> for char {
    fn from(a: AggregatedBook) -> char {
        a as isize as u8 as char
    }
}

impl From<AggregatedBook> for &'static [u8] {
    fn from(a: AggregatedBook) -> &'static [u8] {
        match a {
            AggregatedBook::NO => b"N",
            AggregatedBook::YES => b"Y",
        }
    }
}

impl TryFrom<char> for AggregatedBook {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::AggregatedBook, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum MDEntryType {
    BID = '0' as isize,
    OFFER = '1' as isize,
    TRADE = '2' as isize,
    INDEX_VALUE = '3' as isize,
    OPENING_PRICE = '4' as isize,
    CLOSING_PRICE = '5' as isize,
    SETTLEMENT_PRICE = '6' as isize,
    TRADING_SESSION_HIGH_PRICE = '7' as isize,
    TRADING_SESSION_LOW_PRICE = '8' as isize,
    TRADING_SESSION_VWAP_PRICE = '9' as isize,
}

impl From<MDEntryType> for char {
    fn from(a: MDEntryType) -> char {
        a as isize as u8 as char
    }
}

impl From<MDEntryType> for &'static [u8] {
    fn from(a: MDEntryType) -> &'static [u8] {
        match a {
            MDEntryType::BID => b"0",
            MDEntryType::OFFER => b"1",
            MDEntryType::TRADE => b"2",
            MDEntryType::INDEX_VALUE => b"3",
            MDEntryType::OPENING_PRICE => b"4",
            MDEntryType::CLOSING_PRICE => b"5",
            MDEntryType::SETTLEMENT_PRICE => b"6",
            MDEntryType::TRADING_SESSION_HIGH_PRICE => b"7",
            MDEntryType::TRADING_SESSION_LOW_PRICE => b"8",
            MDEntryType::TRADING_SESSION_VWAP_PRICE => b"9",
        }
    }
}

impl TryFrom<char> for MDEntryType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::BID),
            '1' => Ok(Self::OFFER),
            '2' => Ok(Self::TRADE),
            '3' => Ok(Self::INDEX_VALUE),
            '4' => Ok(Self::OPENING_PRICE),
            '5' => Ok(Self::CLOSING_PRICE),
            '6' => Ok(Self::SETTLEMENT_PRICE),
            '7' => Ok(Self::TRADING_SESSION_HIGH_PRICE),
            '8' => Ok(Self::TRADING_SESSION_LOW_PRICE),
            '9' => Ok(Self::TRADING_SESSION_VWAP_PRICE),
            _ => Err(DecodeError::UnknownChar(Tags::MDEntryType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum TickDirection {
    PLUS_TICK = '0' as isize,
    ZERO_PLUS_TICK = '1' as isize,
    MINUS_TICK = '2' as isize,
    ZERO_MINUS_TICK = '3' as isize,
}

impl From<TickDirection> for char {
    fn from(a: TickDirection) -> char {
        a as isize as u8 as char
    }
}

impl From<TickDirection> for &'static [u8] {
    fn from(a: TickDirection) -> &'static [u8] {
        match a {
            TickDirection::PLUS_TICK => b"0",
            TickDirection::ZERO_PLUS_TICK => b"1",
            TickDirection::MINUS_TICK => b"2",
            TickDirection::ZERO_MINUS_TICK => b"3",
        }
    }
}

impl TryFrom<char> for TickDirection {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::PLUS_TICK),
            '1' => Ok(Self::ZERO_PLUS_TICK),
            '2' => Ok(Self::MINUS_TICK),
            '3' => Ok(Self::ZERO_MINUS_TICK),
            _ => Err(DecodeError::UnknownChar(Tags::TickDirection, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum MDUpdateAction {
    NEW = '0' as isize,
    CHANGE = '1' as isize,
    DELETE = '2' as isize,
}

impl From<MDUpdateAction> for char {
    fn from(a: MDUpdateAction) -> char {
        a as isize as u8 as char
    }
}

impl From<MDUpdateAction> for &'static [u8] {
    fn from(a: MDUpdateAction) -> &'static [u8] {
        match a {
            MDUpdateAction::NEW => b"0",
            MDUpdateAction::CHANGE => b"1",
            MDUpdateAction::DELETE => b"2",
        }
    }
}

impl TryFrom<char> for MDUpdateAction {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::NEW),
            '1' => Ok(Self::CHANGE),
            '2' => Ok(Self::DELETE),
            _ => Err(DecodeError::UnknownChar(Tags::MDUpdateAction, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum MDReqRejReason {
    UNKNOWN_SYMBOL = '0' as isize,
    DUPLICATE_MDREQID = '1' as isize,
    INSUFFICIENT_BANDWIDTH = '2' as isize,
    INSUFFICIENT_PERMISSIONS = '3' as isize,
    UNSUPPORTED_SUBSCRIPTIONREQUESTTYPE = '4' as isize,
    UNSUPPORTED_MARKETDEPTH = '5' as isize,
    UNSUPPORTED_MDUPDATETYPE = '6' as isize,
    UNSUPPORTED_AGGREGATEDBOOK = '7' as isize,
    UNSUPPORTED_MDENTRYTYPE = '8' as isize,
}

impl From<MDReqRejReason> for char {
    fn from(a: MDReqRejReason) -> char {
        a as isize as u8 as char
    }
}

impl From<MDReqRejReason> for &'static [u8] {
    fn from(a: MDReqRejReason) -> &'static [u8] {
        match a {
            MDReqRejReason::UNKNOWN_SYMBOL => b"0",
            MDReqRejReason::DUPLICATE_MDREQID => b"1",
            MDReqRejReason::INSUFFICIENT_BANDWIDTH => b"2",
            MDReqRejReason::INSUFFICIENT_PERMISSIONS => b"3",
            MDReqRejReason::UNSUPPORTED_SUBSCRIPTIONREQUESTTYPE => b"4",
            MDReqRejReason::UNSUPPORTED_MARKETDEPTH => b"5",
            MDReqRejReason::UNSUPPORTED_MDUPDATETYPE => b"6",
            MDReqRejReason::UNSUPPORTED_AGGREGATEDBOOK => b"7",
            MDReqRejReason::UNSUPPORTED_MDENTRYTYPE => b"8",
        }
    }
}

impl TryFrom<char> for MDReqRejReason {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::UNKNOWN_SYMBOL),
            '1' => Ok(Self::DUPLICATE_MDREQID),
            '2' => Ok(Self::INSUFFICIENT_BANDWIDTH),
            '3' => Ok(Self::INSUFFICIENT_PERMISSIONS),
            '4' => Ok(Self::UNSUPPORTED_SUBSCRIPTIONREQUESTTYPE),
            '5' => Ok(Self::UNSUPPORTED_MARKETDEPTH),
            '6' => Ok(Self::UNSUPPORTED_MDUPDATETYPE),
            '7' => Ok(Self::UNSUPPORTED_AGGREGATEDBOOK),
            '8' => Ok(Self::UNSUPPORTED_MDENTRYTYPE),
            _ => Err(DecodeError::UnknownChar(Tags::MDReqRejReason, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum DeleteReason {
    CANCELATION = '0' as isize,
    ERROR = '1' as isize,
}

impl From<DeleteReason> for char {
    fn from(a: DeleteReason) -> char {
        a as isize as u8 as char
    }
}

impl From<DeleteReason> for &'static [u8] {
    fn from(a: DeleteReason) -> &'static [u8] {
        match a {
            DeleteReason::CANCELATION => b"0",
            DeleteReason::ERROR => b"1",
        }
    }
}

impl TryFrom<char> for DeleteReason {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::CANCELATION),
            '1' => Ok(Self::ERROR),
            _ => Err(DecodeError::UnknownChar(Tags::DeleteReason, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum OpenCloseSettleFlag {
    DAILY_OPEN = '0' as isize,
    SESSION_OPEN = '1' as isize,
    DELIVERY_SETTLEMENT_PRICE = '2' as isize,
}

impl From<OpenCloseSettleFlag> for char {
    fn from(a: OpenCloseSettleFlag) -> char {
        a as isize as u8 as char
    }
}

impl From<OpenCloseSettleFlag> for &'static [u8] {
    fn from(a: OpenCloseSettleFlag) -> &'static [u8] {
        match a {
            OpenCloseSettleFlag::DAILY_OPEN => b"0",
            OpenCloseSettleFlag::SESSION_OPEN => b"1",
            OpenCloseSettleFlag::DELIVERY_SETTLEMENT_PRICE => b"2",
        }
    }
}

impl TryFrom<char> for OpenCloseSettleFlag {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::DAILY_OPEN),
            '1' => Ok(Self::SESSION_OPEN),
            '2' => Ok(Self::DELIVERY_SETTLEMENT_PRICE),
            _ => Err(DecodeError::UnknownChar(Tags::OpenCloseSettleFlag, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum FinancialStatus {
    BANKRUPT = '1' as isize,
}

impl From<FinancialStatus> for char {
    fn from(a: FinancialStatus) -> char {
        a as isize as u8 as char
    }
}

impl From<FinancialStatus> for &'static [u8] {
    fn from(a: FinancialStatus) -> &'static [u8] {
        match a {
            FinancialStatus::BANKRUPT => b"1",
        }
    }
}

impl TryFrom<char> for FinancialStatus {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Self::BANKRUPT),
            _ => Err(DecodeError::UnknownChar(Tags::FinancialStatus, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum CorporateAction {
    EX_DIVIDEND = 'A' as isize,
    EX_DISTRIBUTION = 'B' as isize,
    EX_RIGHTS = 'C' as isize,
    NEW = 'D' as isize,
    EX_INTEREST = 'E' as isize,
}

impl From<CorporateAction> for char {
    fn from(a: CorporateAction) -> char {
        a as isize as u8 as char
    }
}

impl From<CorporateAction> for &'static [u8] {
    fn from(a: CorporateAction) -> &'static [u8] {
        match a {
            CorporateAction::EX_DIVIDEND => b"A",
            CorporateAction::EX_DISTRIBUTION => b"B",
            CorporateAction::EX_RIGHTS => b"C",
            CorporateAction::NEW => b"D",
            CorporateAction::EX_INTEREST => b"E",
        }
    }
}

impl TryFrom<char> for CorporateAction {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Self::EX_DIVIDEND),
            'B' => Ok(Self::EX_DISTRIBUTION),
            'C' => Ok(Self::EX_RIGHTS),
            'D' => Ok(Self::NEW),
            'E' => Ok(Self::EX_INTEREST),
            _ => Err(DecodeError::UnknownChar(Tags::CorporateAction, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum QuoteAckStatus {
    ACCEPTED = 0,
    CANCELED_FOR_SYMBOL = 1,
    CANCELED_FOR_SECURITY_TYPE = 2,
    CANCELED_FOR_UNDERLYING = 3,
    CANCELED_ALL = 4,
    REJECTED = 5,
}

impl TryFrom<u8> for QuoteAckStatus {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::ACCEPTED),
            1 => Ok(Self::CANCELED_FOR_SYMBOL),
            2 => Ok(Self::CANCELED_FOR_SECURITY_TYPE),
            3 => Ok(Self::CANCELED_FOR_UNDERLYING),
            4 => Ok(Self::CANCELED_ALL),
            5 => Ok(Self::REJECTED),
            _ => Err(DecodeError::UnknownInt(Tags::QuoteAckStatus, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum QuoteCancelType {
    CANCEL_FOR_SYMBOL = 1,
    CANCEL_FOR_SECURITY_TYPE = 2,
    CANCEL_FOR_UNDERLYING_SYMBOL = 3,
    CANCEL_FOR_ALL_QUOTES = 4,
}

impl TryFrom<u8> for QuoteCancelType {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::CANCEL_FOR_SYMBOL),
            2 => Ok(Self::CANCEL_FOR_SECURITY_TYPE),
            3 => Ok(Self::CANCEL_FOR_UNDERLYING_SYMBOL),
            4 => Ok(Self::CANCEL_FOR_ALL_QUOTES),
            _ => Err(DecodeError::UnknownInt(Tags::QuoteCancelType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum QuoteRejectReason {
    UNKNOWN_SYMBOL = 1,
    EXCHANGE = 2,
    QUOTE_REQUEST_EXCEEDS_LIMIT = 3,
    TOO_LATE_TO_ENTER = 4,
    UNKNOWN_QUOTE = 5,
    DUPLICATE_QUOTE = 6,
    INVALID_BID_ASK_SPREAD = 7,
    INVALID_PRICE = 8,
    NOT_AUTHORIZED_TO_QUOTE_SECURITY = 9,
}

impl TryFrom<u8> for QuoteRejectReason {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::UNKNOWN_SYMBOL),
            2 => Ok(Self::EXCHANGE),
            3 => Ok(Self::QUOTE_REQUEST_EXCEEDS_LIMIT),
            4 => Ok(Self::TOO_LATE_TO_ENTER),
            5 => Ok(Self::UNKNOWN_QUOTE),
            6 => Ok(Self::DUPLICATE_QUOTE),
            7 => Ok(Self::INVALID_BID_ASK_SPREAD),
            8 => Ok(Self::INVALID_PRICE),
            9 => Ok(Self::NOT_AUTHORIZED_TO_QUOTE_SECURITY),
            _ => Err(DecodeError::UnknownInt(Tags::QuoteRejectReason, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum QuoteResponseLevel {
    NO_ACKNOWLEDGEMENT = 0,
    ACKNOWLEDGE_ONLY_NEGATIVE_OR_ERRONEOUS_QUOTES = 1,
    ACKNOWLEDGE_EACH_QUOTE_MESSAGES = 2,
}

impl TryFrom<u8> for QuoteResponseLevel {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::NO_ACKNOWLEDGEMENT),
            1 => Ok(Self::ACKNOWLEDGE_ONLY_NEGATIVE_OR_ERRONEOUS_QUOTES),
            2 => Ok(Self::ACKNOWLEDGE_EACH_QUOTE_MESSAGES),
            _ => Err(DecodeError::UnknownInt(Tags::QuoteResponseLevel, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum QuoteRequestType {
    MANUAL = 1,
    AUTOMATIC = 2,
}

impl TryFrom<u8> for QuoteRequestType {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::MANUAL),
            2 => Ok(Self::AUTOMATIC),
            _ => Err(DecodeError::UnknownInt(Tags::QuoteRequestType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum SecurityRequestType {
    REQUEST_SECURITY_IDENTITY_AND_SPECIFICATIONS = 0,
    REQUEST_SECURITY_IDENTITY_FOR_THE_SPECIFICATIONS_PROVIDED = 1,
    REQUEST_LIST_SECURITY_TYPES = 2,
    REQUEST_LIST_SECURITIES = 3,
}

impl TryFrom<u8> for SecurityRequestType {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::REQUEST_SECURITY_IDENTITY_AND_SPECIFICATIONS),
            1 => Ok(Self::REQUEST_SECURITY_IDENTITY_FOR_THE_SPECIFICATIONS_PROVIDED),
            2 => Ok(Self::REQUEST_LIST_SECURITY_TYPES),
            3 => Ok(Self::REQUEST_LIST_SECURITIES),
            _ => Err(DecodeError::UnknownInt(Tags::SecurityRequestType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum SecurityResponseType {
    ACCEPT_SECURITY_PROPOSAL_AS_IS = 1,
    ACCEPT_SECURITY_PROPOSAL_WITH_REVISIONS_AS_INDICATED_IN_THE_MESSAGE = 2,
    LIST_OF_SECURITY_TYPES_RETURNED_PER_REQUEST = 3,
    LIST_OF_SECURITIES_RETURNED_PER_REQUEST = 4,
    REJECT_SECURITY_PROPOSAL = 5,
    CAN_NOT_MATCH_SELECTION_CRITERIA = 6,
}

impl TryFrom<u8> for SecurityResponseType {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::ACCEPT_SECURITY_PROPOSAL_AS_IS),
            2 => Ok(Self::ACCEPT_SECURITY_PROPOSAL_WITH_REVISIONS_AS_INDICATED_IN_THE_MESSAGE),
            3 => Ok(Self::LIST_OF_SECURITY_TYPES_RETURNED_PER_REQUEST),
            4 => Ok(Self::LIST_OF_SECURITIES_RETURNED_PER_REQUEST),
            5 => Ok(Self::REJECT_SECURITY_PROPOSAL),
            6 => Ok(Self::CAN_NOT_MATCH_SELECTION_CRITERIA),
            _ => Err(DecodeError::UnknownInt(Tags::SecurityResponseType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum UnsolicitedIndicator {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<UnsolicitedIndicator> for char {
    fn from(a: UnsolicitedIndicator) -> char {
        a as isize as u8 as char
    }
}

impl From<UnsolicitedIndicator> for &'static [u8] {
    fn from(a: UnsolicitedIndicator) -> &'static [u8] {
        match a {
            UnsolicitedIndicator::NO => b"N",
            UnsolicitedIndicator::YES => b"Y",
        }
    }
}

impl TryFrom<char> for UnsolicitedIndicator {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::UnsolicitedIndicator, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum SecurityTradingStatus {
    OPENING_DELAY = 1,
    MARKET_ON_CLOSE_IMBALANCE_SELL = 10,
    NO_MARKET_IMBALANCE = 12,
    NO_MARKET_ON_CLOSE_IMBALANCE = 13,
    ITS_PRE_OPENING = 14,
    NEW_PRICE_INDICATION = 15,
    TRADE_DISSEMINATION_TIME = 16,
    READY_TO_TRADE = 17,
    NOT_AVAILABLE_FOR_TRADING = 18,
    NOT_TRADED_ON_THIS_MARKET = 19,
    TRADING_HALT = 2,
    UNKNOWN_OR_INVALID = 20,
    RESUME = 3,
    NO_OPEN_NO_RESUME = 4,
    PRICE_INDICATION = 5,
    TRADING_RANGE_INDICATION = 6,
    MARKET_IMBALANCE_BUY = 7,
    MARKET_IMBALANCE_SELL = 8,
    MARKET_ON_CLOSE_IMBALANCE_BUY = 9,
}

impl TryFrom<u8> for SecurityTradingStatus {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::OPENING_DELAY),
            10 => Ok(Self::MARKET_ON_CLOSE_IMBALANCE_SELL),
            12 => Ok(Self::NO_MARKET_IMBALANCE),
            13 => Ok(Self::NO_MARKET_ON_CLOSE_IMBALANCE),
            14 => Ok(Self::ITS_PRE_OPENING),
            15 => Ok(Self::NEW_PRICE_INDICATION),
            16 => Ok(Self::TRADE_DISSEMINATION_TIME),
            17 => Ok(Self::READY_TO_TRADE),
            18 => Ok(Self::NOT_AVAILABLE_FOR_TRADING),
            19 => Ok(Self::NOT_TRADED_ON_THIS_MARKET),
            2 => Ok(Self::TRADING_HALT),
            20 => Ok(Self::UNKNOWN_OR_INVALID),
            3 => Ok(Self::RESUME),
            4 => Ok(Self::NO_OPEN_NO_RESUME),
            5 => Ok(Self::PRICE_INDICATION),
            6 => Ok(Self::TRADING_RANGE_INDICATION),
            7 => Ok(Self::MARKET_IMBALANCE_BUY),
            8 => Ok(Self::MARKET_IMBALANCE_SELL),
            9 => Ok(Self::MARKET_ON_CLOSE_IMBALANCE_BUY),
            _ => Err(DecodeError::UnknownInt(Tags::SecurityTradingStatus, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum HaltReasonChar {
    NEWS_DISSEMINATION = 'D' as isize,
    ORDER_INFLUX = 'E' as isize,
    ORDER_IMBALANCE = 'I' as isize,
    ADDITIONAL_INFORMATION = 'M' as isize,
    NEWS_PENDING = 'P' as isize,
    EQUIPMENT_CHANGEOVER = 'X' as isize,
}

impl From<HaltReasonChar> for char {
    fn from(a: HaltReasonChar) -> char {
        a as isize as u8 as char
    }
}

impl From<HaltReasonChar> for &'static [u8] {
    fn from(a: HaltReasonChar) -> &'static [u8] {
        match a {
            HaltReasonChar::NEWS_DISSEMINATION => b"D",
            HaltReasonChar::ORDER_INFLUX => b"E",
            HaltReasonChar::ORDER_IMBALANCE => b"I",
            HaltReasonChar::ADDITIONAL_INFORMATION => b"M",
            HaltReasonChar::NEWS_PENDING => b"P",
            HaltReasonChar::EQUIPMENT_CHANGEOVER => b"X",
        }
    }
}

impl TryFrom<char> for HaltReasonChar {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'D' => Ok(Self::NEWS_DISSEMINATION),
            'E' => Ok(Self::ORDER_INFLUX),
            'I' => Ok(Self::ORDER_IMBALANCE),
            'M' => Ok(Self::ADDITIONAL_INFORMATION),
            'P' => Ok(Self::NEWS_PENDING),
            'X' => Ok(Self::EQUIPMENT_CHANGEOVER),
            _ => Err(DecodeError::UnknownChar(Tags::HaltReasonChar, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum InViewOfCommon {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<InViewOfCommon> for char {
    fn from(a: InViewOfCommon) -> char {
        a as isize as u8 as char
    }
}

impl From<InViewOfCommon> for &'static [u8] {
    fn from(a: InViewOfCommon) -> &'static [u8] {
        match a {
            InViewOfCommon::NO => b"N",
            InViewOfCommon::YES => b"Y",
        }
    }
}

impl TryFrom<char> for InViewOfCommon {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::InViewOfCommon, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum DueToRelated {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<DueToRelated> for char {
    fn from(a: DueToRelated) -> char {
        a as isize as u8 as char
    }
}

impl From<DueToRelated> for &'static [u8] {
    fn from(a: DueToRelated) -> &'static [u8] {
        match a {
            DueToRelated::NO => b"N",
            DueToRelated::YES => b"Y",
        }
    }
}

impl TryFrom<char> for DueToRelated {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::DueToRelated, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum Adjustment {
    CANCEL = 1,
    ERROR = 2,
    CORRECTION = 3,
}

impl TryFrom<u8> for Adjustment {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::CANCEL),
            2 => Ok(Self::ERROR),
            3 => Ok(Self::CORRECTION),
            _ => Err(DecodeError::UnknownInt(Tags::Adjustment, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum TradSesMethod {
    ELECTRONIC = 1,
    OPEN_OUTCRY = 2,
    TWO_PARTY = 3,
}

impl TryFrom<u8> for TradSesMethod {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::ELECTRONIC),
            2 => Ok(Self::OPEN_OUTCRY),
            3 => Ok(Self::TWO_PARTY),
            _ => Err(DecodeError::UnknownInt(Tags::TradSesMethod, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum TradSesMode {
    TESTING = 1,
    SIMULATED = 2,
    PRODUCTION = 3,
}

impl TryFrom<u8> for TradSesMode {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::TESTING),
            2 => Ok(Self::SIMULATED),
            3 => Ok(Self::PRODUCTION),
            _ => Err(DecodeError::UnknownInt(Tags::TradSesMode, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum TradSesStatus {
    HALTED = 1,
    OPEN = 2,
    CLOSED = 3,
    PRE_OPEN = 4,
    PRE_CLOSE = 5,
}

impl TryFrom<u8> for TradSesStatus {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::HALTED),
            2 => Ok(Self::OPEN),
            3 => Ok(Self::CLOSED),
            4 => Ok(Self::PRE_OPEN),
            5 => Ok(Self::PRE_CLOSE),
            _ => Err(DecodeError::UnknownInt(Tags::TradSesStatus, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum QuoteEntryRejectReason {
    UNKNOWN_SYMBOL = 1,
    EXCHANGE = 2,
    QUOTE_EXCEEDS_LIMIT = 3,
    TOO_LATE_TO_ENTER = 4,
    UNKNOWN_QUOTE = 5,
    DUPLICATE_QUOTE = 6,
    INVALID_BID_ASK_SPREAD = 7,
    INVALID_PRICE = 8,
    NOT_AUTHORIZED_TO_QUOTE_SECURITY = 9,
}

impl TryFrom<u8> for QuoteEntryRejectReason {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::UNKNOWN_SYMBOL),
            2 => Ok(Self::EXCHANGE),
            3 => Ok(Self::QUOTE_EXCEEDS_LIMIT),
            4 => Ok(Self::TOO_LATE_TO_ENTER),
            5 => Ok(Self::UNKNOWN_QUOTE),
            6 => Ok(Self::DUPLICATE_QUOTE),
            7 => Ok(Self::INVALID_BID_ASK_SPREAD),
            8 => Ok(Self::INVALID_PRICE),
            9 => Ok(Self::NOT_AUTHORIZED_TO_QUOTE_SECURITY),
            _ => Err(DecodeError::UnknownInt(Tags::QuoteEntryRejectReason, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum SessionRejectReason {
    INVALID_TAG_NUMBER = 0,
    REQUIRED_TAG_MISSING = 1,
    SENDINGTIME_ACCURACY_PROBLEM = 10,
    INVALID_MSGTYPE = 11,
    TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE = 2,
    UNDEFINED_TAG = 3,
    TAG_SPECIFIED_WITHOUT_A_VALUE = 4,
    VALUE_IS_INCORRECT = 5,
    INCORRECT_DATA_FORMAT_FOR_VALUE = 6,
    DECRYPTION_PROBLEM = 7,
    SIGNATURE_PROBLEM = 8,
    COMPID_PROBLEM = 9,
}

impl TryFrom<u8> for SessionRejectReason {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::INVALID_TAG_NUMBER),
            1 => Ok(Self::REQUIRED_TAG_MISSING),
            10 => Ok(Self::SENDINGTIME_ACCURACY_PROBLEM),
            11 => Ok(Self::INVALID_MSGTYPE),
            2 => Ok(Self::TAG_NOT_DEFINED_FOR_THIS_MESSAGE_TYPE),
            3 => Ok(Self::UNDEFINED_TAG),
            4 => Ok(Self::TAG_SPECIFIED_WITHOUT_A_VALUE),
            5 => Ok(Self::VALUE_IS_INCORRECT),
            6 => Ok(Self::INCORRECT_DATA_FORMAT_FOR_VALUE),
            7 => Ok(Self::DECRYPTION_PROBLEM),
            8 => Ok(Self::SIGNATURE_PROBLEM),
            9 => Ok(Self::COMPID_PROBLEM),
            _ => Err(DecodeError::UnknownInt(Tags::SessionRejectReason, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum BidRequestTransType {
    CANCEL = 'C' as isize,
    NO = 'N' as isize,
}

impl From<BidRequestTransType> for char {
    fn from(a: BidRequestTransType) -> char {
        a as isize as u8 as char
    }
}

impl From<BidRequestTransType> for &'static [u8] {
    fn from(a: BidRequestTransType) -> &'static [u8] {
        match a {
            BidRequestTransType::CANCEL => b"C",
            BidRequestTransType::NO => b"N",
        }
    }
}

impl TryFrom<char> for BidRequestTransType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'C' => Ok(Self::CANCEL),
            'N' => Ok(Self::NO),
            _ => Err(DecodeError::UnknownChar(Tags::BidRequestTransType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum SolicitedFlag {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<SolicitedFlag> for char {
    fn from(a: SolicitedFlag) -> char {
        a as isize as u8 as char
    }
}

impl From<SolicitedFlag> for &'static [u8] {
    fn from(a: SolicitedFlag) -> &'static [u8] {
        match a {
            SolicitedFlag::NO => b"N",
            SolicitedFlag::YES => b"Y",
        }
    }
}

impl TryFrom<char> for SolicitedFlag {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::SolicitedFlag, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum ExecRestatementReason {
    GT_CORPORATE_ACTION = 0,
    GT_RENEWAL = 1,
    VERBAL_CHANGE = 2,
    REPRICING_OF_ORDER = 3,
    BROKER_OPTION = 4,
    PARTIAL_DECLINE_OF_ORDERQTY = 5,
}

impl TryFrom<u8> for ExecRestatementReason {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::GT_CORPORATE_ACTION),
            1 => Ok(Self::GT_RENEWAL),
            2 => Ok(Self::VERBAL_CHANGE),
            3 => Ok(Self::REPRICING_OF_ORDER),
            4 => Ok(Self::BROKER_OPTION),
            5 => Ok(Self::PARTIAL_DECLINE_OF_ORDERQTY),
            _ => Err(DecodeError::UnknownInt(Tags::ExecRestatementReason, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum BusinessRejectReason {
    OTHER = 0,
    UNKOWN_ID = 1,
    UNKNOWN_SECURITY = 2,
    UNSUPPORTED_MESSAGE_TYPE = 3,
    APPLICATION_NOT_AVAILABLE = 4,
    CONDITIONALLY_REQUIRED_FIELD_MISSING = 5,
}

impl TryFrom<u8> for BusinessRejectReason {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::OTHER),
            1 => Ok(Self::UNKOWN_ID),
            2 => Ok(Self::UNKNOWN_SECURITY),
            3 => Ok(Self::UNSUPPORTED_MESSAGE_TYPE),
            4 => Ok(Self::APPLICATION_NOT_AVAILABLE),
            5 => Ok(Self::CONDITIONALLY_REQUIRED_FIELD_MISSING),
            _ => Err(DecodeError::UnknownInt(Tags::BusinessRejectReason, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum MsgDirection {
    RECEIVE = 'R' as isize,
    SEND = 'S' as isize,
}

impl From<MsgDirection> for char {
    fn from(a: MsgDirection) -> char {
        a as isize as u8 as char
    }
}

impl From<MsgDirection> for &'static [u8] {
    fn from(a: MsgDirection) -> &'static [u8] {
        match a {
            MsgDirection::RECEIVE => b"R",
            MsgDirection::SEND => b"S",
        }
    }
}

impl TryFrom<char> for MsgDirection {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'R' => Ok(Self::RECEIVE),
            'S' => Ok(Self::SEND),
            _ => Err(DecodeError::UnknownChar(Tags::MsgDirection, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum DiscretionInst {
    RELATED_TO_DISPLAYED_PRICE = '0' as isize,
    RELATED_TO_MARKET_PRICE = '1' as isize,
    RELATED_TO_PRIMARY_PRICE = '2' as isize,
    RELATED_TO_LOCAL_PRIMARY_PRICE = '3' as isize,
    RELATED_TO_MIDPOINT_PRICE = '4' as isize,
    RELATED_TO_LAST_TRADE_PRICE = '5' as isize,
}

impl From<DiscretionInst> for char {
    fn from(a: DiscretionInst) -> char {
        a as isize as u8 as char
    }
}

impl From<DiscretionInst> for &'static [u8] {
    fn from(a: DiscretionInst) -> &'static [u8] {
        match a {
            DiscretionInst::RELATED_TO_DISPLAYED_PRICE => b"0",
            DiscretionInst::RELATED_TO_MARKET_PRICE => b"1",
            DiscretionInst::RELATED_TO_PRIMARY_PRICE => b"2",
            DiscretionInst::RELATED_TO_LOCAL_PRIMARY_PRICE => b"3",
            DiscretionInst::RELATED_TO_MIDPOINT_PRICE => b"4",
            DiscretionInst::RELATED_TO_LAST_TRADE_PRICE => b"5",
        }
    }
}

impl TryFrom<char> for DiscretionInst {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Self::RELATED_TO_DISPLAYED_PRICE),
            '1' => Ok(Self::RELATED_TO_MARKET_PRICE),
            '2' => Ok(Self::RELATED_TO_PRIMARY_PRICE),
            '3' => Ok(Self::RELATED_TO_LOCAL_PRIMARY_PRICE),
            '4' => Ok(Self::RELATED_TO_MIDPOINT_PRICE),
            '5' => Ok(Self::RELATED_TO_LAST_TRADE_PRICE),
            _ => Err(DecodeError::UnknownChar(Tags::DiscretionInst, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum LiquidityIndType {
    FIVE_DAY_MOVING_AVERAGE = 1,
    TWENTY_DAY_MOVING_AVERAGE = 2,
    NORMAL_MARKET_SIZE = 3,
    OTHER = 4,
}

impl TryFrom<u8> for LiquidityIndType {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::FIVE_DAY_MOVING_AVERAGE),
            2 => Ok(Self::TWENTY_DAY_MOVING_AVERAGE),
            3 => Ok(Self::NORMAL_MARKET_SIZE),
            4 => Ok(Self::OTHER),
            _ => Err(DecodeError::UnknownInt(Tags::LiquidityIndType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum ExchangeForPhysical {
    NO = 'N' as isize,
    YES = 'Y' as isize,
}

impl From<ExchangeForPhysical> for char {
    fn from(a: ExchangeForPhysical) -> char {
        a as isize as u8 as char
    }
}

impl From<ExchangeForPhysical> for &'static [u8] {
    fn from(a: ExchangeForPhysical) -> &'static [u8] {
        match a {
            ExchangeForPhysical::NO => b"N",
            ExchangeForPhysical::YES => b"Y",
        }
    }
}

impl TryFrom<char> for ExchangeForPhysical {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'N' => Ok(Self::NO),
            'Y' => Ok(Self::YES),
            _ => Err(DecodeError::UnknownChar(Tags::ExchangeForPhysical, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum ProgRptReqs {
    BUYSIDE_EXPLICITLY_REQUESTS_STATUS_USING_STATUSREQUEST = 1,
    SELLSIDE_PERIODICALLY_SENDS_STATUS_USING_LISTSTATUS_PERIOD_OPTIONALLY_SPECIFIED_IN_PROGRESSPERIOD =
        2,
    REAL_TIME_EXECUTION_REPORTS = 3,
}

impl TryFrom<u8> for ProgRptReqs {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
    1 => Ok(Self::BUYSIDE_EXPLICITLY_REQUESTS_STATUS_USING_STATUSREQUEST),2 => Ok(Self::SELLSIDE_PERIODICALLY_SENDS_STATUS_USING_LISTSTATUS_PERIOD_OPTIONALLY_SPECIFIED_IN_PROGRESSPERIOD),3 => Ok(Self::REAL_TIME_EXECUTION_REPORTS),
    _=> Err(DecodeError::UnknownInt(Tags::ProgRptReqs, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum IncTaxInd {
    NET = 1,
    GROSS = 2,
}

impl TryFrom<u8> for IncTaxInd {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::NET),
            2 => Ok(Self::GROSS),
            _ => Err(DecodeError::UnknownInt(Tags::IncTaxInd, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum TradeType {
    AGENCY = 'A' as isize,
    VWAP_GUARANTEE = 'G' as isize,
    GUARANTEED_CLOSE = 'J' as isize,
    RISK_TRADE = 'R' as isize,
}

impl From<TradeType> for char {
    fn from(a: TradeType) -> char {
        a as isize as u8 as char
    }
}

impl From<TradeType> for &'static [u8] {
    fn from(a: TradeType) -> &'static [u8] {
        match a {
            TradeType::AGENCY => b"A",
            TradeType::VWAP_GUARANTEE => b"G",
            TradeType::GUARANTEED_CLOSE => b"J",
            TradeType::RISK_TRADE => b"R",
        }
    }
}

impl TryFrom<char> for TradeType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Self::AGENCY),
            'G' => Ok(Self::VWAP_GUARANTEE),
            'J' => Ok(Self::GUARANTEED_CLOSE),
            'R' => Ok(Self::RISK_TRADE),
            _ => Err(DecodeError::UnknownChar(Tags::TradeType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum BasisPxType {
    CLOSING_PRICE_AT_MORNING_SESSION = '2' as isize,
    CLOSING_PRICE = '3' as isize,
    CURRENT_PRICE = '4' as isize,
    SQ = '5' as isize,
    VWAP_THROUGH_A_DAY = '6' as isize,
    VWAP_THROUGH_A_MORNING_SESSION = '7' as isize,
    VWAP_THROUGH_AN_AFTERNOON_SESSION = '8' as isize,
    VWAP_THROUGH_A_DAY_EXCEPT_YORI = '9' as isize,
    VWAP_THROUGH_A_MORNING_SESSION_EXCEPT_YORI = 'A' as isize,
    VWAP_THROUGH_AN_AFTERNOON_SESSION_EXCEPT_YORI = 'B' as isize,
    STRIKE = 'C' as isize,
    OPEN = 'D' as isize,
    OTHERS = 'Z' as isize,
}

impl From<BasisPxType> for char {
    fn from(a: BasisPxType) -> char {
        a as isize as u8 as char
    }
}

impl From<BasisPxType> for &'static [u8] {
    fn from(a: BasisPxType) -> &'static [u8] {
        match a {
            BasisPxType::CLOSING_PRICE_AT_MORNING_SESSION => b"2",
            BasisPxType::CLOSING_PRICE => b"3",
            BasisPxType::CURRENT_PRICE => b"4",
            BasisPxType::SQ => b"5",
            BasisPxType::VWAP_THROUGH_A_DAY => b"6",
            BasisPxType::VWAP_THROUGH_A_MORNING_SESSION => b"7",
            BasisPxType::VWAP_THROUGH_AN_AFTERNOON_SESSION => b"8",
            BasisPxType::VWAP_THROUGH_A_DAY_EXCEPT_YORI => b"9",
            BasisPxType::VWAP_THROUGH_A_MORNING_SESSION_EXCEPT_YORI => b"A",
            BasisPxType::VWAP_THROUGH_AN_AFTERNOON_SESSION_EXCEPT_YORI => b"B",
            BasisPxType::STRIKE => b"C",
            BasisPxType::OPEN => b"D",
            BasisPxType::OTHERS => b"Z",
        }
    }
}

impl TryFrom<char> for BasisPxType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(Self::CLOSING_PRICE_AT_MORNING_SESSION),
            '3' => Ok(Self::CLOSING_PRICE),
            '4' => Ok(Self::CURRENT_PRICE),
            '5' => Ok(Self::SQ),
            '6' => Ok(Self::VWAP_THROUGH_A_DAY),
            '7' => Ok(Self::VWAP_THROUGH_A_MORNING_SESSION),
            '8' => Ok(Self::VWAP_THROUGH_AN_AFTERNOON_SESSION),
            '9' => Ok(Self::VWAP_THROUGH_A_DAY_EXCEPT_YORI),
            'A' => Ok(Self::VWAP_THROUGH_A_MORNING_SESSION_EXCEPT_YORI),
            'B' => Ok(Self::VWAP_THROUGH_AN_AFTERNOON_SESSION_EXCEPT_YORI),
            'C' => Ok(Self::STRIKE),
            'D' => Ok(Self::OPEN),
            'Z' => Ok(Self::OTHERS),
            _ => Err(DecodeError::UnknownChar(Tags::BasisPxType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum PriceType {
    PERCENTAGE = 1,
    PER_SHARE = 2,
    FIXED_AMOUNT = 3,
}

impl TryFrom<u8> for PriceType {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::PERCENTAGE),
            2 => Ok(Self::PER_SHARE),
            3 => Ok(Self::FIXED_AMOUNT),
            _ => Err(DecodeError::UnknownInt(Tags::PriceType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum GTBookingInst {
    BOOK_OUT_ALL_TRADES_ON_DAY_OF_EXECUTION = 0,
    ACCUMULATE_EXECUTIONS_UNTIL_ORDER_IS_FILLED_OR_EXPIRES = 1,
    ACCUMULATE_UNTIL_VERBALLY_NOTIFIED_OTHERWISE = 2,
}

impl TryFrom<u8> for GTBookingInst {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Self::BOOK_OUT_ALL_TRADES_ON_DAY_OF_EXECUTION),
            1 => Ok(Self::ACCUMULATE_EXECUTIONS_UNTIL_ORDER_IS_FILLED_OR_EXPIRES),
            2 => Ok(Self::ACCUMULATE_UNTIL_VERBALLY_NOTIFIED_OTHERWISE),
            _ => Err(DecodeError::UnknownInt(Tags::GTBookingInst, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum NetGrossInd {
    NET = 1,
    GROSS = 2,
}

impl TryFrom<u8> for NetGrossInd {
    type Error = DecodeError;
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            1 => Ok(Self::NET),
            2 => Ok(Self::GROSS),
            _ => Err(DecodeError::UnknownInt(Tags::NetGrossInd, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum ListExecInstType {
    IMMEDIATE = '1' as isize,
    WAIT_FOR_EXECUTE_INSTRUCTION = '2' as isize,
}

impl From<ListExecInstType> for char {
    fn from(a: ListExecInstType) -> char {
        a as isize as u8 as char
    }
}

impl From<ListExecInstType> for &'static [u8] {
    fn from(a: ListExecInstType) -> &'static [u8] {
        match a {
            ListExecInstType::IMMEDIATE => b"1",
            ListExecInstType::WAIT_FOR_EXECUTE_INSTRUCTION => b"2",
        }
    }
}

impl TryFrom<char> for ListExecInstType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Self::IMMEDIATE),
            '2' => Ok(Self::WAIT_FOR_EXECUTE_INSTRUCTION),
            _ => Err(DecodeError::UnknownChar(Tags::ListExecInstType, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum CxlRejResponseTo {
    ORDER_CANCEL_REQUEST = '1' as isize,
    ORDER_CANCEL_REPLACE_REQUEST = '2' as isize,
}

impl From<CxlRejResponseTo> for char {
    fn from(a: CxlRejResponseTo) -> char {
        a as isize as u8 as char
    }
}

impl From<CxlRejResponseTo> for &'static [u8] {
    fn from(a: CxlRejResponseTo) -> &'static [u8] {
        match a {
            CxlRejResponseTo::ORDER_CANCEL_REQUEST => b"1",
            CxlRejResponseTo::ORDER_CANCEL_REPLACE_REQUEST => b"2",
        }
    }
}

impl TryFrom<char> for CxlRejResponseTo {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Self::ORDER_CANCEL_REQUEST),
            '2' => Ok(Self::ORDER_CANCEL_REPLACE_REQUEST),
            _ => Err(DecodeError::UnknownChar(Tags::CxlRejResponseTo, c)),
        }
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum MultiLegReportingType {
    SINGLE_SECURITY = '1' as isize,
    INDIVIDUAL_LEG_OF_A_MULTI_LEG_SECURITY = '2' as isize,
    MULTI_LEG_SECURITY = '3' as isize,
}

impl From<MultiLegReportingType> for char {
    fn from(a: MultiLegReportingType) -> char {
        a as isize as u8 as char
    }
}

impl From<MultiLegReportingType> for &'static [u8] {
    fn from(a: MultiLegReportingType) -> &'static [u8] {
        match a {
            MultiLegReportingType::SINGLE_SECURITY => b"1",
            MultiLegReportingType::INDIVIDUAL_LEG_OF_A_MULTI_LEG_SECURITY => b"2",
            MultiLegReportingType::MULTI_LEG_SECURITY => b"3",
        }
    }
}

impl TryFrom<char> for MultiLegReportingType {
    type Error = DecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Self::SINGLE_SECURITY),
            '2' => Ok(Self::INDIVIDUAL_LEG_OF_A_MULTI_LEG_SECURITY),
            '3' => Ok(Self::MULTI_LEG_SECURITY),
            _ => Err(DecodeError::UnknownChar(Tags::MultiLegReportingType, c)),
        }
    }
}
