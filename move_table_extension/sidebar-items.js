initSidebarItems({"enum":[["TableOperation","A table operation, for supporting cost calculation."]],"fn":[["table_natives","Returns all natives for tables."]],"struct":[["NativeTableContext","The native table context extension. This needs to be attached to the NativeContextExtensions value which is passed into session functions, so its accessible from natives of this extension."],["TableChange","A change of a single table."],["TableChangeSet","A table change set."],["TableHandle","The representation of a table handle. This is created from truncating a sha3-256 based hash over a transaction hash provided by the environment and a table creation counter local to the transaction."]],"trait":[["TableResolver","A table resolver which needs to be provided by the environment. This allows to lookup data in remote storage, as well as retrieve cost of table operations."]]});