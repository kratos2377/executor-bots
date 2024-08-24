use anchor_spl::token::spl_token::solana_program::pubkey::{Pub, Pubkey};
use num_bigint::BigInt;




// pub type MappedRecord<A extends Record<String, unknown>, B> = {
// 	[K in keyof A]: B;
// };

// # Utility Types / Enums / Constants

pub enum ExchangeStatus {
	ACTIVE = 0,
	DEPOSIT_PAUSED = 1,
	WITHDRAW_PAUSED = 2,
	AMM_PAUSED = 4,
	FILL_PAUSED = 8,
	LIQ_PAUSED = 16,
	FUNDING_PAUSED = 32,
	SETTLE_PNL_PAUSED = 64,
	PAUSED = 127,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MarketStatus {
    Initialized,
    Active,
    FundingPaused,
    AmmPaused,
    FillPaused,
    WithdrawPaused,
    ReduceOnly,
    Settlement,
    Delisted,
}



pub enum SpotOperation {
	UPDATE_CUMULATIVE_INTEREST = 1,
	FILL = 2,
	DEPOSIT = 4,
	WITHDRAW = 8,
	LIQUIDATION = 16,
}


pub enum UserStatus {
	BEING_LIQUIDATED = 1,
	BANKRUPT = 2,
	REDUCE_ONLY = 4,
	ADVANCED_LP = 8,
}



pub enum ContractTier {
	  A,
	  B,
      C,
	  SPECULATIVE,
	  HIGHLY_SPECULATIVE,
	  ISOLATED
}

pub enum AssetTier {
	 COLLATERAL,
	 PROTECTED,
	 CROSS,
	 ISOLATED,
      UNLISTED,
}

pub enum SwapDirection {
	 ADD, 
	 REMOVE
}

pub enum SpotBalanceType {
	 DEPOSIT,
	 BORROW,
}

pub enum PositionDirection {
	 LONG, 
	 SHORT,
}

pub enum DepositDirection {
	 DEPOSIT, 
	 WITHDRAW,
}

pub enum OracleSource {
	 PYTH,
	 PYTH_1K, 
	 PYTH_1M,
	 PYTH_PULL,
	 PYTH_1K_PULL, 
	 PYTH_1M_PULL,
	
	 QUOTE_ASSET, 
	 PYTH_STABLE_COIN,
	 PYTH_STABLE_COIN_PULL, 

}

pub enum OrderType {
	 LIMIT,
	 TRIGGER_MARKET, 
	 TRIGGER_LIMIT,
	 MARKET,
	 ORACLE, 
}

pub enum MarketTypeStr { 
    // Perp will be added later
    Spot
 }
pub enum MarketType {
	 SPOT, 
}

pub enum OrderStatus {
	 INIT,
	 OPEN
}
pub enum OrderAction {
	 PLACE,
	 CANCEL,
	 EXPIRE,
	 FILL,
	 TRIGGER,
}

pub enum OrderActionExplanation {
	 NONE,
	 INSUFFICIENT_FREE_COLLATERAL,
	 ORACLE_PRICE_BREACHED_LIMIT_PRICE,
	 MARKET_ORDER_FILLED_TO_LIMIT_PRICE,
	 ORDER_EXPIRED, 
	 LIQUIDATION,  
	 ORDER_FILLED_WITH_AMM,
	 ORDER_FILLED_WITH_AMM_JIT, 
	 ORDER_FILLED_WITH_AMM_JIT_LP_SPLIT,
	 ORDER_FILLED_WITH_LP_JIT, 
	 ORDER_FILLED_WITH_MATCH, 
	 ORDER_FILLED_WITH_MATCH_JIT,
	 MARKET_EXPIRED,
	 RISK_INCREASING_ORDER,
	 ORDER_FILLED_WITH_SERUM,
	 REDUCE_ONLY_ORDER_INCREASED_POSITION,
	 DERISK_LP,
}

pub enum OrderTriggerCondition {
	 ABOVE,
	 BELOW,
	 TRIGGERED_ABOVE,
	 TRIGGERED_BELOW ,
}

pub enum SpotFulfillmentType {
	 EXTERNAL,
	 MATCH,
}

pub enum SpotFulfillmentStatus {
	 ENABLED,
	 DISABLED,
}

pub enum DepositExplanation {
	 NONE,
	 TRANSFER,
	 BORROW,
	 REPAY_BORROW,
}

pub enum SettlePnlExplanation {
	 NONE,
	 EXPIRED_POSITION,
}

pub enum SpotFulfillmentConfigStatus {
	 ENABLED,
	 DISABLED,
}

pub enum StakeAction {
	 STAKE,
	 UNSTAKE_REQUEST,
	 UNSTAKE_CANCEL_REQUEST,
	 UNSTAKE,
	 UNSTAKE_TRANSFER,
	 STAKE_TRANSFER,
}

pub enum SettlePnlMode {
	 TRY_SETTLE,
	 MUST_SETTLE,
}

pub fn isVariant(prop_map: &HashMap<String, String>, key: String) -> bool {
	return prop_map.hasOwnProperty(ket);
}

pub fn isOneOfVariant(prop_map: &HashMap<String, String>, types: Vec<String>) {
    types.iter().any(|type_name| prop_map.contains_key(type_name))
}

pub fn getVariant(prop_map: &HashMap<String, String>) -> String {
    prop_map.keys().next().cloned()
}

pub enum TradeSide {
	None = 0,
	Buy = 1,
	Sell = 2,
}

pub enum CandleResolution {
    ONE_MINUTE,
    FIVE_MINUTE,
    FIFTEEN_MINUTE,
    SIXTY_MINUTE,
    TWENTY_FOUR_HOUR,
    DAYS,
    WEEKS,
    MONTHS,

}


pub struct NewUserRecord  {
	ts: BigInt,
	userAuthority: Pubkey,
	user: Pubkey,
	subAccountId: u64,
	name: Vec<u8>,
	referrer: Pubkey,
}

pub struct DepositRecord  {
	ts: BigInt,
	userAuthority: Pubkey,
	user: Pubkey,
	direction: Option<DepositDirection>,
	marketIndex: u64,
	amount: BigInt,
	oraclePrice: BigInt,
	marketDepositBalance: BigInt,
	marketWithdrawBalance: BigInt,
	marketCumulativeDepositInterest: BigInt,
	marketCumulativeBorrowInterest: BigInt,
	totalDepositsAfter: BigInt,
	totalWithdrawsAfter: BigInt,
	depositRecordId: BigInt,
	explanation: DepositExplanation,
	transferUser: Option<Pubkey>,
}

pub struct  SpotInterestRecord  {
	ts: BigInt,
	marketIndex: u64,
	depositBalance: BigInt,
	cumulativeDepositInterest: BigInt,
	borrowBalance: BigInt,
	cumulativeBorrowInterest: BigInt,
	optimalUtilization: u64,
	optimalBorrowRate: u64,
	maxBorrowRate: u64,
}

pub struct  CurveRecord {
	ts: BigInt,
	recordId: BigInt,
	marketIndex: u64,
	pegMultiplierBefore: BigInt,
	baseAssetReserveBefore: BigInt,
	quoteAssetReserveBefore: BigInt,
	sqrtKBefore: BigInt,
	pegMultiplierAfter: BigInt,
	baseAssetReserveAfter: BigInt,
	quoteAssetReserveAfter: BigInt,
	sqrtKAfter: BigInt,
	baseAssetAmountLong: BigInt,
	baseAssetAmountShort: BigInt,
	baseAssetAmountWithAmm: BigInt,
	totalFee: BigInt,
	totalFeeMinusDistributions: BigInt,
	adjustmentCost: BigInt,
	numberOfUsers: BigInt,
	oraclePrice: BigInt,
	fillRecord: BigInt,
}



pub struct LPRecord  {
	ts: BigInt,
	user: Pubkey,
	action: LPAction,
	nShares: BigInt,
	marketIndex: u64,
	deltaBaseAssetAmount: BigInt,
	deltaQuoteAssetAmount: BigInt,
	pnl: BigInt,
}

pub enum LPAction {
	 ADD_LIQUIDITY ,
	 REMOVE_LIQUIDITY,
	 SETTLE_LIQUIDITY,
	 REMOVE_LIQUIDITY_DERISK,
}

pub struct FundingRateRecord {
	ts: BigInt,
	recordId: BigInt,
	marketIndex: u64,
	fundingRate: BigInt,
	fundingRateLong: BigInt,
	fundingRateShort: BigInt,
	cumulativeFundingRateLong: BigInt,
	cumulativeFundingRateShort: BigInt,
	oraclePriceTwap: BigInt,
	markPriceTwap: BigInt,
	periodRevenue: BigInt,
	baseAssetAmountWithAmm: BigInt,
	baseAssetAmountWithUnsettledLp: BigInt,
}

pub struct FundingPaymentRecord {
	ts: BigInt,
	userAuthority: Pubkey,
	user: Pubkey,
	marketIndex: u64,
	fundingPayment: BigInt,
	baseAssetAmount: BigInt,
	userLastCumulativeFunding: BigInt,
	ammCumulativeFundingLong: BigInt,
	ammCumulativeFundingShort: BigInt,
}

pub struct LiquidationRecord  {
	ts: BigInt,
	user: Pubkey,
	liquidator: Pubkey,
	liquidationType: LiquidationType,
	marginRequirement: BigInt,
	totalCollateral: BigInt,
	marginFreed: BigInt,
	liquidationId: u64,
	bankrupt: bool,
	canceledOrderIds: Vec<BigInt>,
	liquidatePerp: LiquidatePerpRecord,
	liquidateSpot: LiquidateSpotRecord,
	liquidateBorrowForPerpPnl: LiquidateBorrowForPerpPnlRecord,
	liquidatePerpPnlForDeposit: LiquidatePerpPnlForDepositRecord,
	perpBankruptcy: PerpBankruptcyRecord,
	spotBankruptcy: SpotBankruptcyRecord,
}

pub enum LiquidationType {
	 LIQUIDATE_PERP_PNL_FOR_DEPOSIT,
	 SPOT_BANKRUPTCY,
	 LIQUIDATE_SPOT,
}

pub struct LiquidatePerpRecord  {
	marketIndex: u64,
	oraclePrice: BigInt,
	baseAssetAmount: BigInt,
	quoteAssetAmount: BigInt,
	lpShares: BigInt,
	userOrderId: BigInt,
	liquidatorOrderId: BigInt,
	fillRecordId: BigInt,
	liquidatorFee: BigInt,
	ifFee: BigInt,
}

pub struct  LiquidateSpotRecord {
	assetMarketIndex: u64,
	assetPrice: BigInt,
	assetTransfer: BigInt,
	liabilityMarketIndex: u64,
	liabilityPrice: BigInt,
	liabilityTransfer: BigInt,
	ifFee: BigInt,
}


pub struct SpotBankruptcyRecord  {
	marketIndex: u64,
	borrowAmount: BigInt,
	cumulativeDepositInterestDelta: BigInt,
	ifPayment: BigInt,
}

pub struct  SettlePnlRecord {
	ts: BigInt,
	user: Pubkey,
	marketIndex: u64,
	pnl: BigInt,
	baseAssetAmount: BigInt,
	quoteAssetAmountAfter: BigInt,
	quoteEntryAmount: BigInt,
	settlePrice: BigInt,
	explanation: SettlePnlExplanation,
}

pub struct OrderRecord {
	ts: BigInt,
	user: Pubkey,
	order: Order,
}

pub struct OrderActionRecord {
	ts: BigInt,
	action: OrderAction,
	actionExplanation: OrderActionExplanation,
	marketIndex: u64,
	marketType: MarketType,
	filler: Option<Pubkey>,
	fillerReward: Option<BigInt>,
	fillRecordId: Option<BigInt>,
	baseAssetAmountFilled: Option<BigInt>,
	quoteAssetAmountFilled: Option<BigInt>,
	takerFee: Option<BigInt>,
	makerFee: Option<BigInt>,
	referrerReward: Option<u64>,
	quoteAssetAmountSurplus: Option<BigInt>,
	spotFulfillmentMethodFee: Option<BigInt>,
	taker: Option<Pubkey>,
	takerOrderId: Option<u64>,
	takerOrderDirection: Option<PositionDirection>,
	takerOrderBaseAssetAmount: Option<BigInt>,
	takerOrderCumulativeBaseAssetAmountFilled: Option<BigInt>,
	takerOrderCumulativeQuoteAssetAmountFilled: Option<BigInt>,
	maker: Option<Pubkey>,
	makerOrderId: Option<u64>,
	makerOrderDirection: Option<PositionDirection>,
	makerOrderBaseAssetAmount: Option<BigInt>,
	makerOrderCumulativeBaseAssetAmountFilled: Option<BigInt>,
	makerOrderCumulativeQuoteAssetAmountFilled: Option<BigInt>,
	oraclePrice: BigInt,
}

pub struct SwapRecord  {
	ts: BigInt,
	user: Pubkey,
	amountOut: BigInt,
	amountIn: BigInt,
	outMarketIndex: u64,
	inMarketIndex: u64,
	outOraclePrice: BigInt,
	inOraclePrice: BigInt,
	fee: BigInt,
}

pub struct SpotMarketVaultDepositRecord {
	ts: BigInt,
	marketIndex: u64,
	depositBalance: BigInt,
	cumulativeDepositInterestBefore: BigInt,
	cumulativeDepositInterestAfter: BigInt,
	depositTokenAmountBefore: BigInt,
	amount: BigInt,
}

pub struct StateAccount {
	admin: Pubkey,
	exchangeStatus: u64,
	whitelistMint: Pubkey,
	discountMint: Pubkey,
	oracleGuardRails: OracleGuardRails,
	numberOfAuthorities: BigInt,
	numberOfSubAccounts: BigInt,
	numberOfMarkets: u64,
	numberOfSpotMarkets: u64,
	minPerpAuctionDuration: u64,
	defaultMarketOrderTimeInForce: u64,
	defaultSpotAuctionDuration: u64,
	liquidationMarginBufferRatio: u64,
	settlementDuration: u64,
	maxNumberOfSubAccounts: u64,
	signer: Pubkey,
	signerNonce: u64,
	srmVault: Pubkey,
	perpFeeStructure: FeeStructure,
	spotFeeStructure: FeeStructure,
	lpCooldownTime: BigInt,
	initialPctToLiquidate: u64,
	liquidationDuration: u64,
	maxInitializeUserFee: u64,
}

pub struct HistoricalOracleData {
	lastOraclePrice: BigInt,
	lastOracleDelay: BigInt,
	lastOracleConf: BigInt,
	lastOraclePriceTwap: BigInt,
	lastOraclePriceTwap5Min: BigInt,
	lastOraclePriceTwapTs: BigInt,
}

pub struct HistoricalIndexData {
	lastIndexBidPrice: BigInt,
	lastIndexAskPrice: BigInt,
	lastIndexPriceTwap: BigInt,
	lastIndexPriceTwap5Min: BigInt,
	lastIndexPriceTwapTs: BigInt,
}

pub struct SpotMarketAccount {
	status: MarketStatus,
	assetTier: AssetTier,
	name: Vec<u64>,

	marketIndex: u64,
	pubkey: Pubkey,
	mint: Pubkey,
	vault: Pubkey,

	oracle: Pubkey,
	oracleSource: OracleSource,
	historicalOracleData: HistoricalOracleData,
	historicalIndexData: HistoricalIndexData,

	// insuranceFund: {
	// 	vault: Pubkey,
	// 	totalShares: BigInt,
	// 	userShares: BigInt,
	// 	sharesBase: BigInt,
	// 	unstakingPeriod: BigInt,
	// 	lastRevenueSettleTs: BigInt,
	// 	revenueSettlePeriod: BigInt,
	// 	totalFactor: u64,
	// 	userFactor: u64,
	// },

	revenuePool: PoolBalance,

	ifLiquidationFee: u64,
	decimals: u64,
	optimalUtilization: u64,
	optimalBorrowRate: u64,
	maxBorrowRate: u64,
	cumulativeDepositInterest: BigInt,
	cumulativeBorrowInterest: BigInt,
	totalSocialLoss: BigInt,
	totalQuoteSocialLoss: BigInt,
	depositBalance: BigInt,
	borrowBalance: BigInt,
	maxTokenDeposits: BigInt,

	lastInterestTs: BigInt,
	lastTwapTs: BigInt,
	initialAssetWeight: u64,
	maintenanceAssetWeight: u64,
	initialLiabilityWeight: u64,
	maintenanceLiabilityWeight: u64,
	liquidatorFee: u64,
	imfFactor: u64,
	scaleInitialAssetWeightStart: BigInt,

	withdrawGuardThreshold: BigInt,
	depositTokenTwap: BigInt,
	borrowTokenTwap: BigInt,
	utilizationTwap: BigInt,
	nextDepositRecordId: BigInt,

	orderStepSize: BigInt,
	orderTickSize: BigInt,
	minOrderSize: BigInt,
	maxPositionSize: BigInt,
	nextFillRecordId: BigInt,
	spotFeePool: PoolBalance,
	totalSpotFee: BigInt,
	totalSwapFee: BigInt,

	flashLoanAmount: BigInt,
	flashLoanInitialTokenAmount: BigInt,

	ordersEnabled: bool,

	pausedOperations: u64,

	ifPausedOperations: u64,

	maxTokenBorrowsFraction: u64,
	minBorrowRate: u64,

	fuelBoostDeposits: u64,
	fuelBoostBorrows: u64,
	fuelBoostTaker: u64,
	fuelBoostMaker: u64,
	fuelBoostInsurance: u64,

	tokenProgram: u64,
}

pub struct  PoolBalance {
	scaledBalance: BigInt,
	marketIndex: u64,
}

pub struct UserFeeStructure {
    pub totalFeePaid: BigInt,
    pub totalFeeRebate: BigInt,
    pub totalTokenDiscount: BigInt,
    pub totalRefereeDiscount: BigInt,
    pub totalReferrerReward: BigInt,
    pub current_epoch_referrer_reward: BigInt,
}

pub struct UserStatsAccount {
	numberOfSubAccounts: u64,
	numberOfSubAccountsCreated: u64,
	makerVolume30D: BigInt,
	takerVolume30D: BigInt,
	fillerVolume30D: BigInt,
	lastMakerVolume30DTs: BigInt,
	lastTakerVolume30DTs: BigInt,
	lastFillerVolume30DTs: BigInt,
	fees:  UserFeeStructure,
	referrer: Pubkey,
	isReferrer: bool,
	authority: Pubkey,
	ifStakedQuoteAssetAmount: BigInt,

	lastFuelIfBonusUpdateTs: u64, // u32 onchain

	fuelInsurance: u64,
	fuelDeposits: u64,
	fuelBorrows: u64,
	fuelPositions: u64,
	fuelTaker: u64,
	fuelMaker: u64,

	ifStakedGovTokenAmount: BigInt,
}

pub struct UserAccount  {
	authority: Pubkey,
	delegate: Pubkey,
	name: Vec<64>,
	subAccountId: u64,
	spotPositions: Vec<SpotPosition>,
	perpPositions: Vec<PerpPosition>,
	orders: Vec<Order>,
	status: u64,
	nextLiquidationId: u64,
	nextOrderId: u64,
	maxMarginRatio: u64,
	lastAddPerpLpSharesTs: BigInt,
	settledPerpPnl: BigInt,
	totalDeposits: BigInt,
	totalWithdraws: BigInt,
	totalSocialLoss: BigInt,
	cumulativePerpFunding: BigInt,
	cumulativeSpotFees: BigInt,
	liquidationMarginFreed: BigInt,
	lastActiveSlot: BigInt,
	isMarginTradingEnabled: bool,
	idle: bool,
	openOrders: u64,
	hasOpenOrder: bool,
	openAuctions: u64,
	hasOpenAuction: bool,
	lastFuelBonusUpdateTs: u64,
}

pub struct SpotPosition {
	marketIndex: u64,
	balanceType: SpotBalanceType,
	scaledBalance: BigInt,
	openOrders: u64,
	openBids: BigInt,
	openAsks: BigInt,
	cumulativeDeposits: BigInt,
}

pub struct  Order {
	status: OrderStatus,
	orderType: OrderType,
	marketType: MarketType,
	slot: BigInt,
	orderId: u64,
	userOrderId: u64,
	marketIndex: u64,
	price: BigInt,
	baseAssetAmount: BigInt,
	quoteAssetAmount: BigInt,
	baseAssetAmountFilled: BigInt,
	quoteAssetAmountFilled: BigInt,
	direction: PositionDirection,
	reduceOnly: bool,
	triggerPrice: BigInt,
	triggerCondition: OrderTriggerCondition,
	existingPositionDirection: PositionDirection,
	postOnly: bool,
	immediateOrCancel: bool,
	oraclePriceOffset: u64,
	auctionDuration: u64,
	auctionStartPrice: BigInt,
	auctionEndPrice: BigInt,
	maxTs: BigInt,
}

pub struct  OrderParams {
	orderType: OrderType,
	marketType: MarketType,
	userOrderId: u64,
	direction: PositionDirection,
	baseAssetAmount: BigInt,
	price: BigInt,
	marketIndex: u64,
	reduceOnly: bool,
	postOnly: PostOnlyParams,
	immediateOrCancel: bool,
	triggerPrice: Option<BigInt>,
	triggerCondition: OrderTriggerCondition,
	oraclePriceOffset: Option<u64>,
	auctionDuration: Option<u64>,
	maxTs: Option<BigInt>,
	auctionStartPrice: Option<BigInt>,
	auctionEndPrice: Option<BigInt>,
}
pub enum PostOnlyParams {
	 NONE,
	 MUST_POST_ONLY ,
	 TRY_POST_ONLY,
	 SLIDE,
}

pub struct  NecessaryOrderParams {
	orderType: OrderType,
	marketIndex: u64,
	baseAssetAmount: BigInt,
	direction: PositionDirection,
}

pub struct  OptionalOrderParams  {
	order_params: Option<OrderParams>,
    neccesary_params: NecessaryOrderParams,
} 

pub struct  ModifyOrderParams  {
	order_params: Option<OrderParams>,
    modifiy_policy: ModifyOrderPolicy,
} 

pub enum ModifyOrderPolicy {
	 MUST_MODIFY,
	 TRY_MODIFY ,
}


impl OrderParams {
    fn default() -> Self {
            OrderParams {
                orderType: OrderType::Market,
                marketType: MarketType::SPOT,
                userOrderId: 0,
                baseAssetAmount: ZERO,
                marketIndex: 0,
                reduceOnly: false,
                postOnly: PostOnlyParams::None,
                immediateOrCancel: false,
                triggerPrice: None,
                triggerCondition: OrderTriggerCondition::Above,
                oraclePriceOffset: None,
                auctionDuration: None,
                maxTs: None,
                auctionStartPrice: None,
                auctionEndPrice: None,
                direction: PositionDirection::LONG,
                price: 0,
            }
    }
}


pub struct  MakerInfo  {
	maker: Pubkey,
	makerStats: Pubkey,
	makerUserAccount: UserAccount,
	order: Option<Order>,
}

pub struct  TakerInfo {
	taker: Pubkey,
	takerStats: Pubkey,
	takerUserAccount: UserAccount,
	order: Order,
}

pub struct  ReferrerInfo  {
	referrer: Pubkey,
	referrerStats: Pubkey,
}


#[derive(Debug, Clone)]
pub struct BaseTxParams {
    pub compute_units: Option<u64>,        // Optional field
    pub compute_units_price: Option<u64>,  // Optional field
}

pub struct  ProcessingTxParams {
	useSimulatedComputeUnits: Option<bool>,
	computeUnitsBufferMultiplier: Option<u64>,
	useSimulatedComputeUnitsForCUPriceCalculation: Option<bool>,
	getCUPriceFromComputeUnits: Option<u64>,
	lowerBoundCu: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct TxParams {
    pub base_tx_params: BaseTxParams,
    pub processing_tx_params: ProcessingTxParams,
}

pub enum SwapReduceOnly {
	 In ,
	 Out,
}

// # Misc Types
#[async_trait]
pub trait IWallet {
    async fn sign_transaction(&self, tx: &mut Transaction) -> Transaction;
    async fn sign_all_transactions(&self, txs: &mut [Transaction]) -> Vec<Transaction>;
    fn public_key(&self) -> Pubkey;
}


#[async_trait]
pub trait IVersionedWallet {
    async fn sign_versioned_transaction(&self, tx: &mut VersionedTransaction) -> VersionedTransaction;
    
    async fn sign_all_versioned_transactions(&self, txs: &mut [VersionedTransaction]) -> Vec<VersionedTransaction>;

    fn public_key(&self) -> Pubkey;
}

pub struct  FeeStructure {
	feeTiers: Vec<FeeTier>,
	fillerRewardStructure: OrderFillerRewardStructure,
	flatFillerFee: BigInt,
	referrerRewardEpochUpperBound: BigInt,
}

pub struct  FeeTier {
	feeNumerator: u64,
	feeDenominator: u64,
	makerRebateNumerator: u64,
	makerRebateDenominator: u64,
	referrerRewardNumerator: u64,
	referrerRewardDenominator: u64,
	refereeFeeNumerator: u64,
	refereeFeeDenominator: u64,
}

pub struct  OrderFillerRewardStructure {
	rewardNumerator: BigInt,
	rewardDenominator: BigInt,
	timeBasedRewardLowerBound: BigInt,
}

pub struct PriceDivergence {
    markOraclePercentDivergence: BigInt,
    oracleTwap5MinPercentDivergence: BigInt,
}

pub struct OracleValidity {
    
		slotsBeforeStaleForAmm: BigInt,
		slotsBeforeStaleForMargin: BigInt,
		confidenceIntervalMaxSize: BigInt,
		tooVolatileRatio: BigInt,
	
}

pub struct  OracleGuardRails  {
	priceDivergence: PriceDivergence,
	validity: OracleValidity
}

pub struct  PrelaunchOracle {
	price: BigInt,
	maxPrice: BigInt,
	confidence: BigInt,
	ammLastUpdateSlot: BigInt,
	lastUpdateSlot: BigInt,
	perpMarketIndex: u64,
}

pub enum MarginCategory {
    Initial,
    Maintenance,
}



pub struct  SerumV3FulfillmentConfigAccount {
	fulfillmentType: SpotFulfillmentType,
	status: SpotFulfillmentStatus,
	pubkey: Pubkey,
	marketIndex: u64,
	serumProgramId: Pubkey,
	serumMarket: Pubkey,
	serumRequestQueue: Pubkey,
	serumEventQueue: Pubkey,
	serumBids: Pubkey,
	serumAsks: Pubkey,
	serumBaseVault: Pubkey,
	serumQuoteVault: Pubkey,
	serumOpenOrders: Pubkey,
	serumSignerNonce: BigInt,
}



pub struct  ReferrerNameAccount {
	name: Vec<u64>,
	user: Pubkey,
	authority: Pubkey,
	userStats: Pubkey,
}

pub struct  PerpMarketExtendedInfo  {
	marketIndex: u64,
	/**
	 * Min order size measured in base asset, using base precision
	 */
	minOrderSize: BigInt,
	/**
	 * Margin maintenance percentage, using margin precision (1e4)
	 */
	marginMaintenance: u64,
	/**
	 * Max insurance available, measured in quote asset, using quote preicision
	 */
	availableInsurance: BigInt,
	/**
	 * Pnl pool available, this is measured in quote asset, using quote precision.
	 * Should be generated by using getTokenAmount and passing in the scaled balance of the base asset + quote spot account
	 */
	pnlPoolValue: BigInt,
	contractTier: ContractTier,
}

pub struct  HealthComponents  {
	deposits: Vec<HealthComponent>,
	borrows: Vec<HealthComponent>,
	perpPositions: Vec<HealthComponent>,
	perpPnl: Vec<HealthComponent>,
}

pub struct  HealthComponent {
	marketIndex: u64,
	size: BigInt,
	value: BigInt,
	weight: BigInt,
	weightedValue: BigInt,
}

#[derive(Debug, Clone)]
pub struct DriftClientMetricsEvents {
    pub tx_signed: Vec<SignedTxData>, // Array of SignedTxData
    pub pre_tx_signed: (),              // Represents void
}

#[derive(Debug, Clone)]
pub enum SignedTx {
    Transaction(Transaction),
    VersionedTransaction(VersionedTransaction),
}


pub struct  SignedTxData  {
	txSig: String,
	signedTx: SignedTx,
	lastValidBlockHeight: u64,
	blockHash: String,
}
