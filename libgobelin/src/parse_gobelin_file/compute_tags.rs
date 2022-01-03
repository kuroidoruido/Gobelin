use crate::TransactionBucket;

pub fn compute_tags(transactions: &[TransactionBucket]) -> Vec<String> {
    let mut tags: Vec<String> = transactions
        .iter()
        .flat_map(|b| &b.transactions)
        .filter_map(|t| t.tag.clone())
        .filter(|x| x != "<=>")
        .collect::<Vec<_>>();
    tags.dedup();
    tags.sort();
    tags
}
