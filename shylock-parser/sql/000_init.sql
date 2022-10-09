
CREATE TABLE IF NOT EXISTS auctions (
    id TEXT PRIMARY KEY NOT NULL UNIQUE,

    auction_state AuctionState,

    kind AuctionKind,

    claim_quantity DECIMAL(10, 2),

    lots INTEGER DEFAULT 0,

    lot_kind LotAuctionKind,

    management TEXT NULL,

    bidinfo BidInfo,

    start_date DATETIME,

    end_date DATETIME,

    notice TEXT NULL
);

CREATE UNIQUE INDEX idx_auctions_on_id ON auctions(id);

CREATE TABLE IF NOT EXISTS managements (
    code TEXT PRIMARY KEY NOT NULL UNIQUE,

    description TEXT NULL,

    address TEXT NULL,

    telephone TEXT NULL,

    fax TEXT NULL,

    email TEXT NULL
);

CREATE UNIQUE INDEX idx_managements_on_code ON managements(code);

CREATE TABLE IF NOT EXISTS properties (
    address TEXT NULL,

    -- foreign key to auctions table
    auction_id TEXT NOT NULL,

    bidinfo BidInfo NULL,

    catastro_reference TEXT NULL,

    category PropertyCategory,

    charges Decimal(10, 2) NULL,

    city TEXT NULL,

    coordinates TEXT NULL,

    description TEXT NULL,

    owner_status TEXT NULL,

    postal_code TEXT NULL,

    primary_residence TEXT NULL,

    province Province,

    register_inscription TEXT NULL,

    visitable TEXT NULL
);

CREATE UNIQUE INDEX idx_properties_on_auction_id ON properties(auction_id);

CREATE TABLE IF NOT EXISTS vehicles (
    -- foreign key to auctions table
    auction_id TEXT NOT NULL,

    bidinfo BidInfo NULL,

    brand TEXT NULL,

    category VehicleCategory,

    charges DECIMAL(10, 2) NULL,

    description TEXT NULL,

    frame_number TEXT NULL,

    licensed_date DATETIME NULL,
    
    license_plate TEXT NULL,

    localization TEXT NULL,

    model TEXT NULL,

    visitable TEXT NULL
);

CREATE UNIQUE INDEX idx_vehicles_on_auction_id ON vehicles(auction_id);

CREATE TABLE IF NOT EXISTS others (
    additional_information TEXT NULL,

   -- foreign key to auctions table
    auction_id TEXT NOT NULL,

    bidinfo BidInfo NULL,

    category OtherCategory,

    charges DECIMAL(10, 2) NULL,

    description TEXT NULL,

    judicial_title TEXT NULL,

    visitable TEXT NULL
);

CREATE UNIQUE INDEX idx_others_on_auction_id ON others(auction_id);