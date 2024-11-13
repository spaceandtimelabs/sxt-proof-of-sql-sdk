if (!secrets.apiKey) {
  throw Error("API Key Not Found");
}

// NOTE: Do NOT use this apiKey here in production, it is only here to demonstrate secrets.
return Functions.encodeString(secrets.apiKey);
