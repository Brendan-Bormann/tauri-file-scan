import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import './Reset.scss'
import './App.scss';

function App() {
  const [filepath, setFilepath] = useState("");
  const [filepathInput, setFilepathInput] = useState("/home/pooks/test_folder");
  const [searchTerm, setSearchTerm] = useState("");
  const [searchedItems, setSearchedItems] = useState<string[]>([]);

  async function indexDir(dirName: string) {
    setFilepath(dirName);
    const _result = await invoke("index_directory", { dir: dirName })
  }

  async function searchDir(searchTerm: string) {
    setSearchedItems([]);
    const result: string[] = await invoke("search", { query: searchTerm })
    setSearchedItems(result);
  }

  return (
    <div className="container">
      <h1>Tauri File Scanner</h1>
      <hr />

      <div className="features">
        <div className="feature-container">
          <p>Index a directory { filepath ? <span className="subtitle">- {filepath}</span> : '' }</p>
          <div className="input-holder">
            <input
              onChange={(e) => setFilepathInput(e.currentTarget.value)}
              defaultValue={"/home/pooks/test_folder"}
              placeholder="/home/user/test_folder"
            />
            <button onClick={() => indexDir(filepathInput)}>Index</button>
          </div>
        </div>

        <div className="feature-container">
          <p>Search the Directory</p>
          <input
            onChange={(e) => setSearchTerm(e.currentTarget.value)}
            placeholder=""
          />
          <button onClick={() => searchDir(searchTerm)}>Search</button>
        </div>

        <div className="feature-container">
          <h3>Current Directory: { filepath ? <code>{filepath}</code> : <span className="subtitle">Nothing Indexed</span> }</h3>
          <div className="search-results">
            { searchedItems.map((item, index) => <div className="search-item" key={index}>{index + 1}. <code>{item}</code></div>) }
          </div>
        </div>
      </div>

    </div>
  );
}

export default App;
