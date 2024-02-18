fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = prices[0];
    let mut max_profit = 0;
    let mut profit = 0;

    for i in 1..prices.len() {
        let price = prices[i];

        let test_profit = price - min_price;
        if test_profit > profit {
            profit = test_profit;
            max_profit += profit;
            profit = 0;
            min_price = price;
        } else {
            min_price = min_price.min(price);
        }
    }

    return max_profit;
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let result = max_profit(prices);
    println!("Max profit: {}", result);
}
