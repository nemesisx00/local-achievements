use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use chrono::{DateTime, Utc};
use tokio::sync::Mutex;
use tokio::time::sleep;
use super::request::DataRequest;

#[derive(Debug, Default)]
pub struct RateLimiter
{
	capacity: AtomicU64,
	lastUsed: Arc<Mutex<DateTime<Utc>>>,
	lastRefunded: AtomicBool,
	requests: Arc<Mutex<VecDeque<DataRequest>>>,
	used: AtomicU64,
}

impl RateLimiter
{
	pub fn blockingIsEmpty(&self) -> bool
	{
		return self.requests.blocking_lock()
			.is_empty();
	}
	
	async fn decayUses(&self)
	{
		let mut lastUsed = self.lastUsed.lock().await;
		
		let secondsSinceLast = Utc::now().signed_duration_since(*lastUsed).num_seconds();
		if secondsSinceLast > 0
		{
			let used = self.used.load(Ordering::SeqCst);
			if used > 0
			{
				self.used.store(
					match used > secondsSinceLast as u64
					{
						true => used - secondsSinceLast as u64,
						false => 0,
					},
					Ordering::SeqCst
				);
			}
			
			*lastUsed = Utc::now();
		}
	}
	
	pub async fn isEmpty(&self) -> bool
	{
		return self.requests.lock()
			.await
			.is_empty();
	}
	
	pub async fn len(&self) -> usize
	{
		return self.requests.lock()
			.await
			.len();
	}
	
	pub fn new(capacity: impl Into<u64>) -> Self
	{
		return Self
		{
			capacity: AtomicU64::new(capacity.into()),
			..Default::default()
		};
	}
	
	pub async fn next(&self) -> Option<DataRequest>
	{
		self.decayUses().await;
		
		let capacity = self.capacity.load(Ordering::SeqCst);
		let mut used = self.used.load(Ordering::SeqCst);
		
		while used >= capacity
		{
			sleep(Duration::from_secs(1))
				.await;
			
			self.decayUses()
				.await;
			
			used = self.used.load(Ordering::SeqCst);
		}
		
		let mut requests = self.requests.lock()
			.await;
		
		let request = requests.pop_front();
		
		if request.is_some()
		{
			if !self.lastRefunded.load(Ordering::SeqCst)
			{
				let mut lastUsed = self.lastUsed.lock()
					.await;
				
				*lastUsed = Utc::now();
				self.used.store(used + 1, Ordering::SeqCst);
			}
			else
			{
				self.lastRefunded.store(false, Ordering::SeqCst);
			}
		}
		
		return request;
	}
	
	#[allow(unused)]
	pub async fn push(&self, request: DataRequest)
	{
		let mut requests = self.requests.lock()
			.await;
		requests.push_back(request);
		
		requests.make_contiguous()
			.sort();
	}
	
	pub async fn pushAll(&self, requestList: Vec<DataRequest>)
	{
		if !requestList.is_empty()
		{
			let mut requests = self.requests.lock()
				.await;
			
			for request in requestList
			{
				requests.push_back(request);
			}
			
			requests.make_contiguous()
				.sort();
		}
	}
	
	pub async fn refundUse(&self)
	{
		self.lastRefunded.store(true, Ordering::SeqCst);
	}
}
