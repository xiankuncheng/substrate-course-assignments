import React, { useEffect, useState } from "react";
import { Form, Grid } from "semantic-ui-react";

import { useSubstrate } from "./substrate-lib";
import { TxButton } from "./substrate-lib/components";

import KittyCards from "./KittyCards";

export default function Kitties(props) {
  const { api, keyring } = useSubstrate();
  const { accountPair } = props;

  const [kittyIndexes, setKittyIndexes] = useState([]);
  const [kitties, setKitties] = useState([]);
  const [status, setStatus] = useState("");

  useEffect(() => {
    let unsub = null;

    const fetchKittyIndexes = async () => {
      unsub = await api.query.kittiesModule.kittiesCount(async (count) => {
        const kittyIndex = count.value.toJSON();
        if (kittyIndex <= 0) {
          return;
        }
        setKittyIndexes(Array.from(Array(kittyIndex).keys()));
      });
    };

    fetchKittyIndexes();

    return () => {
      unsub && unsub();
    };
  }, [api, keyring, setKitties]);

  useEffect(() => {
    let unsub = null;

    const fetchKitties = async () => {
      unsub = await api.query.kittiesModule.owner.multi(
        kittyIndexes,
        async (owners) => {
          const kittyDNAs = await api.query.kittiesModule.kitties.multi(
            kittyIndexes
          );
          const kitties = kittyIndexes.map((kittyIndex) => ({
            id: kittyIndex,
            dna: kittyDNAs[kittyIndex].value,
            owner: owners[kittyIndex].value.toJSON(),
          }));

          setKitties(kitties);
        }
      );
    };

    fetchKitties();

    return () => {
      unsub && unsub();
    };
  }, [api, keyring, kittyIndexes]);

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
