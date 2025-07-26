/// Enforces the Delta Law: ±2% of global liquidity price.
///
/// # Arguments
/// * `order_price` - Submitted order price
/// * `liquidity_price` - Global liquidity-weighted average (e.g., VWAP)
/// * `max_delta_bps` - Allowed delta in basis points (e.g., 200 = 2%)
///
/// # Returns
/// `true` if price is within allowed bounds
pub fn check_price_delta(order_price: u64, liquidity_price: u64, max_delta_bps: u64) -> bool {
    if liquidity_price == 0 {
        // No liquidity = unsafe to trade
        return false;
    }

    let delta = if order_price > liquidity_price {
        order_price - liquidity_price
    } else {
        liquidity_price - order_price
    };

    let allowed = (liquidity_price as u128 * max_delta_bps as u128) / 10_000;

    delta as u128 <= allowed
}
