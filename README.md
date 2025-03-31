# Stellar Health Point

## Project Overview

Stellar Health Point is a user-centric platform built on Soroban that empowers
individuals to securely log, store, and control their personal health data. It aims to address the privacy concerns,
lack of control, and data silos prevalent in traditional healthcare systems.

**Problem:** Traditional healthcare systems often lack transparency regarding data handling, leave individuals with
little control over their health data, and create isolated data silos that hinder personalized care. Concerns about data
breaches, unauthorized access, and the monetization of health information erode trust in these systems.

**Solution:** Stellar Health Point leverages Soroban's robust smart contract capabilities, combined with encryption, to
provide a secure, private, and user-controlled health data ecosystem. User health data is encrypted before being stored
on Soroban, ensuring that only the user holds the decryption key. This architecture grants individuals complete
ownership and control over their data, allowing them to selectively share it with healthcare providers or researchers as
they deem appropriate.

## Technical Details

Stellar Health Point leverages Soroban's unique capabilities and a client-side Flutter Web App to provide secure,
user-controlled health data management. The system consists of the following components:

* **Soroban Smart Contract (Health Data Management):** We utilize Soroban's temporary storage and event system to manage
  the encrypted health data. The contract structure is as follows:

    * **Primary Hash ID:** Each user possesses a unique primary hash ID stored in their Soroban profile. This serves as
      the root pointer to their data.
    * **Yearly Data Map:** The primary hash ID maps to a `Map<Year, YearHashID>`. This structure allows for efficient
      retrieval of data organized by year.
    * **Hierarchical Hashing:** Each `YearHashID` then points to a similar structure organizing data by month and day,
      ultimately leading to the encrypted health data log for a specific day.  (Year -> Month -> Day -> Encrypted Health
      Data). This layered approach provides scalability and efficient access.
    * **Data Sharing Mechanism:** When a user wants to share their data, the contract stores the doubly-encrypted data (
      using the user's Stellar Secret Key and either RSA or AES) in temporary storage. The hash of this encrypted data
      is then shared with the intended recipient.
    * **Event Emission:** The Soroban contract emits events to notify the user or other stakeholders about important
      activities, such as data sharing requests or successful data retrieval. This leverages Soroban's event system for
      real-time updates.


* **Flutter Web App (Frontend & Encryption):** The Flutter Web App serves as the user interface and handles all
  encryption/decryption processes locally within the user's browser. It is completely client-side, eliminating the need
  for a backend server.

    * **Data Logging:** Users can easily log their health data through the app's intuitive interface.
    * **Client-Side Encryption:** All data is encrypted *before* being sent to the Soroban network. Encryption is
      performed using the user's Stellar secret key. This ensures that only the user can decrypt and view their data.
    * **Data Sharing:** The app facilitates secure data sharing by encrypting the data with a combination of the user's
      Stellar secret key and either RSA or AES. It then interacts with the Soroban contract to store the
      doubly-encrypted data and share the hash.

**Key points:**

* **No Backend:** There is *no* backend server. All logic resides in the Soroban smart contract and the Flutter Web App.
* **Security Focus:**  The primary goal is secure storage and controlled sharing of health data.
* **Hierarchical Data Structure:** The data structure allows for easy browsing of historical data.
* **Stellar Secret Key:**  The user's Stellar Secret Key is essential for data privacy.
* **Temporary Storage for Sharing:**  Soroban's temporary storage is cleverly used for data sharing purposes.

## Deployment

**Prerequisites:**

* Stellar CLI installed
* A funded Stellar testnet account

**Deployment:**

1. **Clone the repository:**
   ```bash
   git clone git@github.com:sillynerd45/stellar-hp.git
   cd frontend
   git pull --all
   ```

2. **Create local keys:**
   ```bash
   stellar keys generate deployer_admin --network testnet --fund
   stellar keys generate contract_admin --network testnet --fund
   ```

3. **Deploy contract:**
   ```bash
    cd contracts/deployer
    make
   ```

4. **Upgrade contract (as needed):**
   ```bash
   cd contracts/stellar-hp
   make upgrade_contract
   ```

5. **Start the frontend:**
   ```bash
   cd frontend
   flutter run --dart-define-from-file=env/stellar.json --web-renderer canvaskit -d web-server --web-port 5000
   ```

   The frontend will be accessible at `http://localhost:5000`.