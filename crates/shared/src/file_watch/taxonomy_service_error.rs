// PURPOSE: WatchServiceError — structured error type for file watch service failures
use crate::domain_error_vo;

domain_error_vo!(WatchServiceError, "Watch Error");
