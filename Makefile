all: pam_mount.rs
	rustc pam_mount.rs

clean:
	rm -f libpam*.so

install: all
	sudo cp libpam*.so /lib/security/pam_mymount.so