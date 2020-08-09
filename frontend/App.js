import React from "react";
import "./App.css";
import TextTruncate from "react-text-truncate";
import debounce from "debounce";

// TODO: add JWT - basically for login
// XXX: https://dev.to/gkoniaris/how-to-securely-store-jwt-tokens-51cf
// TODO: refactor css to scss
// TODO: protobuf for communication between server and client
// TODO: introduce TS
// TODO: fold the bandname input on scroll
// TODO: submit form manually instead of doing a form submit

const data = [
  {
    id: 1,
    name: "Antagonising the Breadseller Antagonising the Breadseller ",
    which: "band",
  },
  { id: 2, name: "The little light that kept on going", which: "album" },
  { id: 3, name: "Authority of a high vis jacket", which: "song" },
  { id: 4, name: "Antagonising the Breadseller", which: "band" },
  { id: 5, name: "The little light that kept on going", which: "album" },
  { id: 6, name: "Authority of a high vis jacket", which: "song" },
  { id: 7, name: "Antagonising the Breadseller", which: "band" },
  { id: 8, name: "The little light that kept on going", which: "album" },
  { id: 9, name: "Authority of a high vis jacket", which: "song" },
  { id: 10, name: "Antagonising the Breadseller", which: "band" },
  { id: 11, name: "The little light that kept on going", which: "album" },
  { id: 12, name: "Authority of a high vis jacket", which: "song" },
];

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
  return (
    <div id="form-container">
      <form method="post" id="input-form" action="/new">
        <input
          id="bandname-input"
          type="text"
          name="name"
          placeholder="Submit your idea..."
          autoComplete="off"
          autoFocus
          required
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
              />
            </div>
            <div className="type-radio">
              <p htmlFor="type-song">Song</p>
              <input type="radio" id="type-song" name="which" value="song" />
            </div>
            <div className="type-radio">
              <p htmlFor="type-album">Album</p>
              <input type="radio" id="type-album" name="which" value="album" />
            </div>
          </div>
          <input type="submit" id="bandname-submit" value="Submit" />
        </div>
      </form>
    </div>
  );
}

function App() {
  return (
    <div id="content">
      <InputForm />
      <Chunks />
    </div>
  );
}

export default App;
