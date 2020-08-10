import React, { useState, useEffect } from "react";
import "./App.css";
import TextTruncate from "react-text-truncate";
import axios from "axios";

// TODO: add JWT - basically for login
// XXX: https://dev.to/gkoniaris/how-to-securely-store-jwt-tokens-51cf
// TODO: refactor css to scss
// TODO: protobuf for communication between server and client
// TODO: introduce TS
// TODO: fold the bandname input on scroll
// TODO: submit form manually instead of doing a form submit

function Chunk(props) {
  return (
    <li className="chunk">
      <div className="name">
        <TextTruncate lines={1} text={props.name} />
      </div>
      <div className="extras">
        <div className={"type-container " + props.which}>
          <p id={props.id}>{props.which}</p>
        </div>
        <a
          className="search"
          href={`https://www.google.com/search?q=${props.name} ${props.which}`}
        >
          Search ->
        </a>
      </div>
    </li>
  );
}

function Chunks(props) {
  // TODO: add https://www.npmjs.com/package/react-infinite-scroll-component
  // https://www.digitalocean.com/community/tutorials/react-react-infinite-scroll

  let { data, dataSetter, newDataSetter, newData } = props;
  //
  // Defines our fetching of the data
  // Got from: https://codesandbox.io/s/jvvkoo8pq3?file=/src/index.js
  // Unclear if the conditional approach here is good
  useEffect(() => {
    if (newData) {
      async function fetchData() {
        const result = await axios("/all");
        dataSetter(result.data);
      }

      console.log(props.data);
      fetchData();
      newDataSetter(false);
    }
  }, [newData]);
  return (
    <ul id="chunks">
      {data.map((entry) => (
        <Chunk
          key={entry.id}
          id={entry.id}
          name={entry.name}
          which={entry.which}
        />
      ))}
    </ul>
  );
}

function InputForm(props) {
  // This handles changes in the input names and form data generation
  const initialFormData = Object.freeze({
    name: "",
    which: "",
  });
  const [formData, updateFormData] = React.useState(initialFormData);

  const handleChange = (e) => {
    updateFormData({
      ...formData,

      // Trimming any whitespace
      [e.target.name]: e.target.value.trim(),
    });
  };

  // Intercept form submission
  function handleSubmit(event) {
    event.preventDefault();
    axios({
      method: "post",
      url: "/new",
      data: formData,
      headers: { "Content-Type": "application/json" },
    })
      .then(function (response) {
        //handle success
        console.log(response);
        props.alertNewData(true);
      })
      .catch(function (response) {
        //handle error
        console.log("Error occured in request!");
      });
  }

  return (
    <div id="form-container">
      <form id="input-form" onSubmit={handleSubmit}>
        <input
          id="bandname-input"
          type="text"
          name="name"
          placeholder="Submit your idea..."
          autoComplete="off"
          autoFocus
          required
          onChange={handleChange}
        />
        <div id="form-options">
          <div id="form-checkboxes">
            <div className="type-radio">
              <p htmlFor="type-band">Band</p>
              <input
                type="radio"
                id="type-band"
                name="which"
                value="band"
                required
                onChange={handleChange}
              />
            </div>
            <div className="type-radio">
              <p htmlFor="type-song">Song</p>
              <input
                type="radio"
                id="type-song"
                name="which"
                value="song"
                onChange={handleChange}
              />
            </div>
            <div className="type-radio">
              <p htmlFor="type-album">Album</p>
              <input
                type="radio"
                id="type-album"
                name="which"
                value="album"
                onChange={handleChange}
              />
            </div>
          </div>
          <input type="submit" id="bandname-submit" value="Submit" />
        </div>
      </form>
    </div>
  );
}

function App() {
  const [newData, setNewData] = useState(true);
  const [data, setData] = useState([]);
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
