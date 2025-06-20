//FUNCTION TO ENDPOINTS IN INDEX.JS
//GETTING INFO FROM DATABASE THROUGH SQL FILES

//GET
const getReceipt = (req, res) => {
  const db = req.app.get('db');
  db.receipt
    .get_receipt()
    .then(receipt => res.status(200).json(receipt))
    .catch(err => console.log(err));
};
const getReceiptId = (req, res) => {
  const db = req.app.get('db');
  const { receipt_id } = req.session.receipt;
  db.receipt
    .get_receipt(receipt_id)
    .then(receipt_id => res.status(200).json(receipt_id))
    .catch(err => console.log(err));
};
//POST
const addReceipt = (req, res) => {
  const db = req.app.get('db');
  const { receipt } = req.body;
  db.post_receipt(receipt)
    .then(addedReceipt => res.status(200).json(addedReceipt))
    .catch(err => console.log(err));
};
//DELETE
const deleteReceipt = (req, res) => {
  const db = req.app.get('db');
  const receipt_id = +req.param.receipt_id;

  db.delete_receipt(receipt_id).then(response => {
    res.status(200).json(response);
  });
};
//PUT
const updateReceipt = (req, res) => {
  const db = req.app.get('db');
  const { receipt } = req.body;
  const trip_id = +req.params.trip_id;

  db.updateReceipt([receipt, trip_id]).then(response => {
    res.status(200).json(response);
  });
};

module.exports = {
  getReceipt,
  getReceiptId,
  addReceipt,
  deleteReceipt,
  updateReceipt
};
