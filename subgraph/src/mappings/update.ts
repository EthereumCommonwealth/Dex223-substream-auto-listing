import { BigInt, Address, Bytes } from "@graphprotocol/graph-ts";
import { ListingContractUpdated as ListingContractUpdatedEvent } from "../pb/auto_listing/v1/ListingContractUpdated";
import { AutoListing } from "../../generated/schema";

import { ADDRESS_ZERO, ZERO_BI } from "../constants";
import { log } from "@graphprotocol/graph-ts";

export function handleListingContractUpdated(
  event: ListingContractUpdatedEvent
): void {
  const autoListingAddress = Address.fromString(event.autoListing);
  const metadata = Bytes.fromUint8Array(event.meta);
  const name = event.name;
  const owner = Address.fromString(event.owner);
  const url = event.url;
  const lastUpdated = BigInt.fromString(event.timestamp.toString());
  if (
    owner.equals(Address.fromString(ADDRESS_ZERO)) ||
    owner.equals(autoListingAddress) ||
    autoListingAddress.equals(Address.fromString(ADDRESS_ZERO))
  ) {
    log.error("Owner address is invalid {}", [owner.toHexString()]);
    return;
  }
  log.debug("AutoListing address {} and owner address {}", [
    autoListingAddress.toHexString(),
    owner.toHexString(),
  ]);

  let autoListing = AutoListing.load(autoListingAddress.toHexString());

  if (autoListing == null) {
    autoListing = new AutoListing(autoListingAddress.toHexString());
    autoListing.totalTokens = ZERO_BI;
    autoListing.name = name;
  }
  autoListing.lastUpdated = lastUpdated;
  autoListing.url = url;
  autoListing.meta = metadata;
  autoListing.owner = owner.toHexString();
  autoListing.save();
}
