.DELETE_ON_ERROR:

TZDB_VERSION := tzdb-2023c

src/generated/mod.rs: tmp/${TZDB_VERSION}/usr/share/zoneinfo/ tzdb.tar.lz.sha
	cd make-tzdb && cargo r -- ../$(@D) ../$< ../tzdb.tar.lz.sha
	cargo +nightly fmt -- $(@D)/mod.rs $(@D)/by_name.rs $(@D)/raw_tzdata.rs $(@D)/test_all_names.rs

tmp/${TZDB_VERSION}/usr/share/zoneinfo/: tmp/${TZDB_VERSION}/
	cd tmp/${TZDB_VERSION}/ && make PACKRATDATA=backzone PACKRATLIST=zone.tab TOPDIR="." install

tmp/${TZDB_VERSION}/: tmp/${TZDB_VERSION}.tar.lz
	cd tmp/ && tar xf $(<F)

tmp/${TZDB_VERSION}.tar.lz: tzdb.tar.lz.sha | tmp/
	sha512sum -c tzdb.tar.lz.sha || curl -s -o $@ https://data.iana.org/time-zones/releases/$(@F)
	sha512sum -c tzdb.tar.lz.sha

tmp/:
	mkdir -p $@
