import { Events } from "./pb/auto_listing/v1/Events";
import { Protobuf } from "as-proto/assembly";
import { log } from "@graphprotocol/graph-ts";
import {
  handleListingContractUpdated,
  handleListingPrice,
  handleTokenListed,
  handleConverterEventERC20,
  handleConverterEventERC223,
} from "./mappings";

import { ERC20WrapperCreatedEvent } from "./pb/auto_listing/v1/ERC20WrapperCreatedEvent";
import { ERC223WrapperCreatedEvent } from "./pb/auto_listing/v1/ERC223WrapperCreatedEvent";

import { ListingContractUpdated } from "./pb/auto_listing/v1/ListingContractUpdated";
import { ListingPrice } from "./pb/auto_listing/v1/ListingPrice";
import { TokenListed } from "./pb/auto_listing/v1/TokenListed";

function processEvents<T>(events: Array<T>, handler: (event: T) => void): void {
  if (events === null || events.length === 0) {
    return;
  }
  for (let i = 0; i < events.length; i++) {
    log.info("Processing event {}", [i.toString()]);
    handler(events[i]);
  }
}

export function handleEvents(bytes: Uint8Array): void {
  // Декодируем события из полученного байтового массива
  const eventsProto: Events = Protobuf.decode<Events>(bytes, Events.decode);
  if (eventsProto === null) {
    log.error("Failed to decode events", []);
    return;
  }

  const listingContractUpdateds = eventsProto.listingContractUpdateds;
  const listingPrice = eventsProto.listingPrices;
  const tokenListeds = eventsProto.tokenListeds;
  const erc20WrapperCreatedEvents = eventsProto.erc223ToErc20;
  const erc223WrapperCreatedEvents = eventsProto.erc20ToErc223;

  processEvents<ListingContractUpdated>(
    listingContractUpdateds,
    handleListingContractUpdated
  );
  processEvents<ListingPrice>(listingPrice, handleListingPrice);
  processEvents<TokenListed>(tokenListeds, handleTokenListed);
  processEvents<ERC20WrapperCreatedEvent>(
    erc20WrapperCreatedEvents,
    handleConverterEventERC20
  );
  processEvents<ERC223WrapperCreatedEvent>(
    erc223WrapperCreatedEvents,
    handleConverterEventERC223
  );
}
