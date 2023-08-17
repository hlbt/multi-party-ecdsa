#on macos
ios:
	# cargo build --release --target aarch64-apple-ios --no-default-features --features curv-kzen/num-bigint
	# cargo build --release -Z build-std --target armv7s-apple-ios
	# cargo build --release -Z build-std --target armv7-apple-ios
	# strip -AnuSXx target/aarch64-apple-ios/release/libmulti_party_ecdsa.a
	# strip -AnuSXx target/armv7s-apple-ios/release/libmulti_party_ecdsa.a
	# strip -AnuSXx target/armv7-apple-ios/release/libmulti_party_ecdsa.a
	# lipo -create -output "target/libmulti_party_ecdsa.a" "target/aarch64-apple-ios/release/libmulti_party_ecdsa.a" "target/armv7s-apple-ios/release/libmulti_party_ecdsa.a" "target/armv7-apple-ios/release/libmulti_party_ecdsa.a"
	cargo lipo --release --targets aarch64-apple-ios --no-default-features --features curv-kzen/num-bigint

#on linux
android:
	cargo build --target aarch64-linux-android --release --no-default-features --features curv-kzen/num-bigint
	cargo build --target armv7-linux-androideabi --release --no-default-features --features curv-kzen/num-bigint
	cargo build --target i686-linux-android  --release --no-default-features --features curv-kzen/num-bigint