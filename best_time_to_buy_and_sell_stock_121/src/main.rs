fn max_profit_naive(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    //let mut buy = 0;
    //let mut sell = 0;

    for (i, price) in prices.iter().enumerate() {
        let max = prices[i..]
            .iter()
            .enumerate()
            .max_by_key(|(_, &v)| v)
            .unwrap();
        let try_profit = max.1 - price;
        if try_profit > profit {
            //buy = i;
            //sell = min.0;
            profit = try_profit;
        }
    }

    return profit;
}

fn max_profit_a_bit_better(prices: Vec<i32>) -> i32 {
    let mut min_price = prices[0];
    let mut max_profit = 0;

    for i in 1..prices.len() {
        let price = prices[i];
        max_profit = max_profit.max(price - min_price);
        min_price = min_price.min(price);
    }

    return max_profit;
}

fn main() {
    let prices = vec![7, 10, 1, 5, 3, 6, 4];
    let profit = max_profit_naive(prices);
    println!("Profit: {}", profit);

    let prices = vec![7, 10, 1, 5, 3, 6, 4];
    let profit = max_profit_a_bit_better(prices);
    println!("Profit: {}", profit);
}
