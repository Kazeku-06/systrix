// src/plugins.rs
//! Plugin system for extending Systrix functionality.
//!
//! Provides both compile-time and optional dynamic plugin loading.

use anyhow::Result;
use std::collections::HashMap;

/// Plugin trait that all plugins must implement
pub trait Plugin: Send + Sync {
    /// Plugin name
    fn name(&self) -> &str;
    
    /// Plugin version
    fn version(&self) -> &str;
    
    /// Initialize the plugin
    fn init(&mut self) -> Result<()>;
    
    /// Execute plugin with given arguments
    fn execute(&self, args: &[String]) -> Result<String>;
    
    /// Cleanup plugin resources
    fn cleanup(&mut self) -> Result<()>;
}

/// Plugin registry for managing plugins
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }
    
    /// Register a plugin
    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> Result<()> {
        let name = plugin.name().to_string();
        self.plugins.insert(name, plugin);
        Ok(())
    }
    
    /// Get a plugin by name
    pub fn get(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }
    
    /// List all registered plugins
    pub fn list(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }
    
    /// Execute a plugin
    pub fn execute(&self, name: &str, args: &[String]) -> Result<String> {
        match self.get(name) {
            Some(plugin) => plugin.execute(args),
            None => anyhow::bail!("Plugin '{}' not found", name),
        }
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Example built-in plugin
pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn name(&self) -> &str {
        "example"
    }
    
    fn version(&self) -> &str {
        "0.1.0"
    }
    
    fn init(&mut self) -> Result<()> {
        Ok(())
    }
    
    fn execute(&self, args: &[String]) -> Result<String> {
        Ok(format!("Example plugin executed with args: {:?}", args))
    }
    
    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
}

#[cfg(feature = "dynamic-plugins")]
pub mod dynamic {
    use super::*;
    use libloading::{Library, Symbol};
    
    /// Load a dynamic plugin from a shared library
    /// 
    /// # Safety
    /// This function loads and executes code from a dynamic library.
    /// Only load plugins from trusted sources.
    pub unsafe fn load_plugin(path: &str) -> Result<Box<dyn Plugin>> {
        let lib = Library::new(path)?;
        
        // Look for plugin_create symbol
        let create: Symbol<unsafe extern "C" fn() -> *mut dyn Plugin> = 
            lib.get(b"plugin_create")?;
        
        let plugin = Box::from_raw(create());
        
        // Prevent library from being unloaded
        std::mem::forget(lib);
        
        Ok(plugin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_registry() {
        let mut registry = PluginRegistry::new();
        let plugin = Box::new(ExamplePlugin);
        
        registry.register(plugin).unwrap();
        
        assert!(registry.get("example").is_some());
        assert_eq!(registry.list(), vec!["example"]);
        
        let result = registry.execute("example", &["test".to_string()]).unwrap();
        assert!(result.contains("test"));
    }
}
