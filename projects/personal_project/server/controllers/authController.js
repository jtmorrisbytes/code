const bcrypt = require('bcryptjs');

const register = (req, res) => {
  const db = req.app.get('db');
  const { user_email, first_name, password } = req.body;
  console.log('req.body register line 6', req.body);
  bcrypt
    .hash(password, 12)
    .then(hash => {
      console.log('authCont line 9 password hashed');
      db.auth.register_user([user_email, first_name, hash]).then(user => {
        req.session.user = {
          id: user[0].user_id,
          user_email: user[0].user_email,
          first_name: user[0].first_name
        };
        res.status(200).json(req.session.user);
      });
    })
    .catch(error => {
      console.log(error);
      res.status(500).json('Error');
    })
    .catch(error => {
      console.log(error);
      res.status(500).json('Error');
    });
};

const login = (req, res) => {
  const db = req.app.get('db');
  const { user_email, password } = req.body;
  db.auth.get_user(user_email).then(user => {
    if (user.length === 0) {
      res.status(400).json('user is not exist');
    } else {
      bcrypt.compare(password, user[0].hash).then(areEqual => {
        if (areEqual) {
          const { user_id, first_name, user_email } = user[0];
          req.session.user = {
            user_id,
            first_name,
            user_email
          };
          res.status(200).json(req.session.user);
        } else {
          res.status(403).json('incorrect email or password');
        }
      });
    }
  });
};

const logOut = (req, res) => {
  req.session.destroy();
  res.status(200).json('User Logged Out');
};

const get_user = (req, res) => {
  // console.log(req.session);
  res.status(200).json(req.session.user);
};

module.exports = {
  register,
  login,
  logOut,
  get_user
};
