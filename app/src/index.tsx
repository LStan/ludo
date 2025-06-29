import React from "react";
import ReactDOM from "react-dom/client";

import "./index.css";

import App from "./App";

import '@fontsource/medievalsharp';

import { WalletContextProvider } from "./contexts/wallet";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);
root.render(
  <React.StrictMode>
    <WalletContextProvider>
        <App />
    </WalletContextProvider>
  </React.StrictMode>
);
