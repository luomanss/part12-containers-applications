const express = require("express");
const router = express.Router();
const redis = require("../redis");

router.get("/", async (_, res) => {
  const addedTodos = await redis.getAsync("addedTodos");

  res.send({
    addedTodos: parseInt(addedTodos) || 0,
  });
});

module.exports = router;