use anchor_lang::prelude::*;
use constant_product_curve::CurveError;

#[error_code]
pub enum CustomError {
    #[msg("DefaultError")]
    DefaultError,
    #[msg("Offer expired.")]
    OfferExpired,
    #[msg("This pool is locked.")]
    PoolLocked,
    #[msg("Slippage exceeded.")]
    SlippageExceeded,
    #[msg("Overflow detected.")]
    Overflow,
    #[msg("Underflow detected.")]
    Underflow,
    #[msg("Invalid token.")]
    InvalidToken,
    #[msg("Actual liquidity is less than minimum.")]
    LiquidityLessThanMinimum,
    #[msg("No liquidity in pool.")]
    NoLiquidityInPool,
    #[msg("Bump error.")]
    BumpError,
    #[msg("Curve error.")]
    CurveError,
    #[msg("Fee is greater than 100%. This is not a very good deal.")]
    InvalidFee,
    #[msg("Invalid update authority.")]
    InvalidAuthority,
    #[msg("No update authority set.")]
    NoAuthoritySet,
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Invalid precision.")]
    InvalidPrecision,
    #[msg("Insufficient balance.")]
    InsufficientBalance,
    #[msg("Zero balance.")]
    ZeroBalance,
    #[msg("No rewards available to claim")]
    NoRewards,
    
}

impl From<CurveError> for CustomError {
    fn from(error: CurveError) -> CustomError {
        match error {
            CurveError::InvalidPrecision => CustomError::InvalidPrecision,
            CurveError::Overflow => CustomError::Overflow,
            CurveError::Underflow => CustomError::Underflow,
            CurveError::InvalidFeeAmount => CustomError::InvalidFee,
            CurveError::InsufficientBalance => CustomError::InsufficientBalance,
            CurveError::ZeroBalance => CustomError::ZeroBalance,
            CurveError::SlippageLimitExceeded => CustomError::SlippageExceeded,
        }
    }
}