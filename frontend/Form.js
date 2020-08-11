import React from "react";
import axios from "axios";

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

export default InputForm;
