#![feature(globs)]
#![feature(phase)]
#[phase(syntax, link)] extern crate log;

#[allow(non_camel_case_types)] 

mod c {
	pub use std::libc::{c_int, c_char, size_t, uint32_t};

	pub type crypt_device = uint;
	
	pub static CRYPT_ANY_SLOT: c_int = -1;
	#[link(name = "cryptsetup")]
	extern "C" {
		// int crypt_init 	(struct crypt_device **cd, const char *device)
		pub fn crypt_init(cd: **crypt_device, device: *c_char) -> c_int;
		// int crypt_load(struct crypt_device *cd, const char *requested_type, void *params )
		pub fn crypt_load(cd: *crypt_device, requested_type: *c_char, params: *c_char) -> c_int;
		
		// int crypt_activate_by_passphrase(struct crypt_device *cd,
		// const char *name, int keyslot, const char *passphrase,
		// size_t  	passphrase_size, uint32_t flags )
		pub fn crypt_activate_by_passphrase(cd: *crypt_device, name: *c_char, keyslot: c_int, 
			passphrase: *c_char, passphrase_size: size_t, flags: uint32_t) -> c_int;

		// int crypt_deactivate (struct crypt_device *cd, const char *name )
		pub fn crypt_deactivate(cd: *crypt_device, name: *c_char) -> c_int;

		pub fn crypt_free(cd: *crypt_device);
	}	
}

mod d {

	use c::*;
	use std::ptr;
	use std::result::Result;

	#[deriving(Show)]
	pub enum ContainerFormat {
		LOOPAES, LUKS1, PLAIN, TCRYPT
	}

	#[deriving(Show)]
	#[allow(raw_pointer_deriving)]
	pub struct CryptoMounter {
		priv cd: *crypt_device,
		priv dm_name: ~str
	}

	impl CryptoMounter {

		pub fn new(container: &str, container_format: ContainerFormat, dm_name: &str) -> Result<~CryptoMounter, int> {
			let cd: *crypt_device = ptr::null();

			let r = container.to_c_str().with_ref(|device|{
				unsafe {crypt_init(&cd, device)}
			});

			let cm = ~CryptoMounter {cd: cd, dm_name: dm_name.to_owned()};
			if r == 0 { cm.load(container_format) } else {Err(r as int)}
		}

		fn load(~self, container_format: ContainerFormat) -> Result<~CryptoMounter, int> {
			let r = container_format.to_str().to_c_str().with_ref(|requested_type|{
				unsafe {crypt_load(self.cd, requested_type, ptr::null())}
			});

			debug!("initialising {}: {}", self, r);
			if r == 0 {Ok(self) } else {Err(r as int)}
		}

		pub fn unlock(~self, password: &str) -> Result<~CryptoMounter, int> {
			let r = self.dm_name.to_c_str().with_ref(|name| {
				password.to_c_str().with_ref(|passphrase| {
					unsafe {
						crypt_activate_by_passphrase(self.cd, name, CRYPT_ANY_SLOT, 
						passphrase, password.len() as size_t, 0)
					}
				})
			});
			debug!("unlocking {}: {}", self, r);
			if r == 0 {Ok(self) } else {Err(r as int)}
		}

		pub fn lock(~self) -> int {
			let r = self.dm_name.to_c_str().with_ref(|name|{
				unsafe {crypt_deactivate(self.cd, name)}
			}) as int;
			debug!("locking {}: {}", self, r);
			r
		}
	}

	impl Drop for CryptoMounter {
    	fn drop(&mut self) {
    		unsafe {crypt_free(self.cd)}
		}
	}
}

fn main() {
	let cm = d::CryptoMounter::new("file.bin", d::LUKS1, "home").and_then(|cm|{
		cm.unlock("preved")
	});
	println!("{}", cm);

	cm.unwrap().lock();
}	

