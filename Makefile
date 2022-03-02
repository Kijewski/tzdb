.DELETE_ON_ERROR:

TZDB_VERSION := tzdb-2021e

tzdb/src/generated.rs: tmp/${TZDB_VERSION}/usr/share/zoneinfo/
	cargo r --bin make-tzdb -- $@ $<
	cargo fmt

tmp/${TZDB_VERSION}/usr/share/zoneinfo/: tmp/${TZDB_VERSION}/
	cd tmp/${TZDB_VERSION}/ && make PACKRATDATA=backzone TOPDIR="." install

tmp/${TZDB_VERSION}/: tmp/${TZDB_VERSION}.tar.lz
	cd tmp/ && tar xf $(<F)

tmp/${TZDB_VERSION}.tar.lz: | tmp/
	curl -s -o $@ https://data.iana.org/time-zones/releases/$(@F)
	sha512sum -c $(@F).sha

tmp/:
	mkdir -p $@
