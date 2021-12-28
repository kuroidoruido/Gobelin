use crate::TransactionBucket;

pub fn compute_tags(transactions: &Vec<TransactionBucket>) -> Vec<String> {
    let mut tags: Vec<String> = transactions
        .iter()
        .flat_map(|b| &b.transactions)
        .map(|t| t.tag.clone())
        .filter(Option::is_some)
        .map(Option::unwrap)
        .filter(|x| x != "<=>")
        .collect::<Vec<_>>();
    tags.dedup();
    tags.sort();
    return tags;
}
