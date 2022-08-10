.DELETE_ON_ERROR:

TZDB_VERSION := tzdb-2022a

src/generated/mod.rs: tmp/${TZDB_VERSION}/usr/share/zoneinfo/
	cargo r --package make-tzdb --bin make-tzdb -- $(@D) $<
	cargo +nightly fmt -- \
		$(@D)/mod.rs \
		$(@D)/raw_tzdata.rs \
		$(@D)/time_zone.rs \
		$(@D)/tzdata.rs

tmp/${TZDB_VERSION}/usr/share/zoneinfo/: tmp/${TZDB_VERSION}/
	cd tmp/${TZDB_VERSION}/ && make PACKRATDATA=backzone TOPDIR="." install

tmp/${TZDB_VERSION}/: tmp/${TZDB_VERSION}.tar.lz
	cd tmp/ && tar xf $(<F)

tmp/${TZDB_VERSION}.tar.lz: | tmp/
	sha512sum -c $(@F).sha || curl -s -o $@ https://data.iana.org/time-zones/releases/$(@F)
	sha512sum -c $(@F).sha

tmp/:
	mkdir -p $@
