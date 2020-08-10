import React, { useState, useEffect } from "react";
import "./App.css";
import Chunks from "./Chunks.js";
import InputForm from "./Form.js";

// TODO: add JWT - basically for login
// XXX: https://dev.to/gkoniaris/how-to-securely-store-jwt-tokens-51cf
// TODO: refactor css to scss
// TODO: protobuf for communication between server and client
// TODO: introduce TS
// TODO: fold the bandname input on scroll
// TODO: submit form manually instead of doing a form submit
// TODO: add greyed out chunk component to happen on load.

function App() {
  const [newData, setNewData] = useState(true);
  const [data, setData] = useState([{ name: "Bla", id: 10, which: "band" }]);
  return (
    <div id="content">
      <InputForm alertNewData={setNewData} />
      <Chunks
        data={data}
        dataSetter={setData}
        newData={newData}
        newDataSetter={setNewData}
      />
    </div>
  );
}

export default App;
