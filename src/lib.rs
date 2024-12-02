//! This module serves as the main library for the changelog manager tool.
//!
//! It provides several submodules to handle different aspects of changelog management:
//!
//! - `create`: Contains functionality to create new changelog entries.
//! - `entry`: Defines the structure and manipulation of individual changelog entries.
//! - `fs_manager`: Handles file system operations related to changelog management (internal use).
//! - `git_info`: Retrieves and processes information from the Git repository.
//! - `merge`: Provides tools to merge multiple changelog entries into a single document.
pub mod create;
pub mod entry;
mod fs_manager;
pub mod git_info;
pub mod merge;
