import React, { useEffect, useState } from "react";
import { Form, Grid } from "semantic-ui-react";

import { useSubstrate } from "./substrate-lib";
import { TxButton } from "./substrate-lib/components";

import KittyCards from "./KittyCards";

export default function Kitties(props) {
  const { api, keyring } = useSubstrate();
  const { accountPair } = props;

  const [kitties, setKitties] = useState([]);
  const [status, setStatus] = useState("");

  const fetchKitties = () => {
    let unsub = null;
    const asyncFetch = async () => {
      unsub = await api.query.kittiesModule.kitties.entries(
        async (kittyEntries) => {
          const kittiess = await Promise.all(
            kittyEntries.map(async ([id, dna], idx) => {
              const owner = await api.query.kittiesModule.owner(idx);
              return { id: idx, dna: dna.value, owner: owner.value.toJSON() };
            })
          );

          setKitties(kittiess);
        }
      );
    };

    asyncFetch();

    return () => {
      unsub && unsub();
    };
  };

  useEffect(fetchKitties, [api, keyring]);

  return (
    <Grid.Column width={16}>
      <h1>小毛孩</h1>
      <KittyCards
        kitties={kitties}
        accountPair={accountPair}
        setStatus={setStatus}
      />
      <Form style={{ margin: "1em 0" }}>
        <Form.Field style={{ textAlign: "center" }}>
          <TxButton
            accountPair={accountPair}
            label="创建小毛孩"
            type="SIGNED-TX"
            setStatus={setStatus}
            attrs={{
              palletRpc: "kittiesModule",
              callable: "create",
              inputParams: [],
              paramFields: [],
            }}
          />
        </Form.Field>
      </Form>
      <div style={{ overflowWrap: "break-word" }}>{status}</div>
    </Grid.Column>
  );
}
