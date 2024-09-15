use std::marker::PhantomData;

use derive_new::new;

use crate::persistence::dynamodb::DynamoDB;

#[derive(new)]
pub struct DynamoDBRepositoryImpl<T> {
    dynamo_db: DynamoDB,
    _marker: PhantomData<T>,
}
