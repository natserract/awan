
import { AppBar, Toolbar, Typography, Tooltip, Box } from '@mui/material'
import Container from '@mui/material/Container';
import Link from 'next/link'
import { makeStyles } from '@mui/styles'

const useStyles = makeStyles(() => ({
  root: {
    flexGrow: 1,
  },
  menuButton: {
    marginRight: 10,
  },
  title: {
    flexGrow: 1,
    textDecoration: 'none',
    paddingTop: 15,
    textTransform: 'uppercase',
    color: '#fff',
  },
  brand: {
    display: 'block',
    background: 'transparent url(aws_logo.png) no-repeat scroll 0 0',
    border: 0,
    width: 59,
    height: 35,
    textIndent: '-9999px',
  },
  appBar: {
    background: '#232f3e !important',
    minHeight: '65px',
  },
  menuContainer: {
    margin: 0,
    padding: 0,
    listStyle: 'none',

    "& li": {
      display: 'inline-block',
      textAlign: 'center',

      "& a": {
        color: '#eaeaea',
        textTransform: 'capitalize',
        padding: '0 16px',
        lineHeight: '72px',
      }
    },
  }
}));

type HeaderProps = {
  id: string;
}

const Header: React.FC<HeaderProps> = (props) => {
  const classes = useStyles();

  const renderMenu = () => {
    const items = ['objects', 'get presigned url', 'upload'];

    return (
      <ul className={classes.menuContainer}>
        {items.map((menu, idx) => (
          <li key={idx}>
            <Typography component="a">{menu}</Typography>
          </li>
        ))}
      </ul>
    )
  };

  return (
    <AppBar position="static" color="transparent" className={classes.appBar}>
      <Toolbar id={props.id}>
        <Container maxWidth="md">
          <Link href="/">
            <a href="https://aws.amazon.com/?nc2=h_lg" className={classes.title}>
              <span className={classes.brand}>Click here to return to Amazon Web Services homepage</span>
            </a>
          </Link>
        </Container>

        <Box sx={{ flexGrow: 1 }} />
        <Box sx={{ display: { xs: 'none', md: 'flex' } }}>
          {renderMenu()}
        </Box>
      </Toolbar>
    </AppBar>
  )
}

export default Header;