// ===============================
// delta_checker.rs : Domex Delta Law (Global Liquidity Enforcement)
// ===============================

/// Enforces the 2% Delta Law based on global liquidity valuation.
///
/// In Domex, an order can only execute if its price is within ±`max_delta_bps`
/// of the liquidity-weighted reference price (e.g., VWAP or trusted oracle).
///
/// # Arguments
/// - `order_price`: The price submitted in the order
/// - `liquidity_price`: The global liquidity-weighted price
/// - `max_delta_bps`: The maximum allowed deviation, in basis points (e.g., 200 = 2%)
///
/// # Returns
/// - `true` if the price is within the allowed delta range, else `false`
pub fn check_price_delta(order_price: u64, liquidity_price: u64, max_delta_bps: u64) -> bool {
    if liquidity_price == 0 {
        // Cannot enforce rule without reference price
        return false;
    }

    let delta = if order_price > liquidity_price {
        order_price - liquidity_price
    } else {
        liquidity_price - order_price
    };

    // Convert max_delta_bps into absolute price movement allowed
    let allowed = (liquidity_price as u128 * max_delta_bps as u128) / 10_000;

    delta as u128 <= allowed
}
