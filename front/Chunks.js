import React, { useEffect } from "react";
import "./Chunks.css";
import TextTruncate from "react-text-truncate";
import axios from "axios";

const handleEasterEgg = (e, which) => {
  let typeLabel = e.currentTarget;
  typeLabel.innerHTML = "Hi!";
  setTimeout(() => {
    typeLabel.innerHTML = which;
  }, 2000);
};

function Chunk(props) {
  return (
    <li className="chunk-container">
      <div className="chunk">
        <div className="name">
          <TextTruncate lines={1} text={props.name} />
        </div>
        <div className="extras">
          <div
            onClick={(e) => handleEasterEgg(e, props.which)}
            className={"type-container " + props.which}
          >
            <p id={props.id}>{props.which}</p>
          </div>
          <a
            className="search"
            href={`https://www.google.com/search?q=${props.name} ${props.which}`}
          >
            Search ->
          </a>
        </div>
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

export default Chunks;
