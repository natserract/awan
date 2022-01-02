import React from 'react';
import Header from './header';
import { makeStyles } from '@mui/styles'
import styles from './styles'
import Container from '@mui/material/Container';
import useScrollTrigger from '@mui/material/useScrollTrigger';
import Box from '@mui/material/Box';
import Fab from '@mui/material/Fab';
import KeyboardArrowUpIcon from '@mui/icons-material/KeyboardArrowUp';
import Zoom from '@mui/material/Zoom';

type ScrollTopProps = {
  children: React.ReactNode,
  window?: Function
}

function ScrollTop(props: ScrollTopProps) {
  const { children, window } = props;
  // Note that you normally won't need to set the window ref as useScrollTrigger
  // will default to window.
  // This is only being set here because the demo is in an iframe.
  const trigger = useScrollTrigger({
    target: window ? window() : undefined,
    disableHysteresis: true,
    threshold: 100,
  });

  const handleClick = (event: unknown) => {
    const anchor = (event.target.ownerDocument || document).querySelector(
      '#back-to-top-anchor',
    );

    if (anchor) {
      anchor.scrollIntoView({
        behavior: 'smooth',
        block: 'center',
      });
    }
  };

  return (
    <Zoom in={trigger}>
      <Box
        onClick={handleClick}
        role="presentation"
        sx={{ position: 'fixed', bottom: 16, right: 16 }}
      >
        {children}
      </Box>
    </Zoom>
  );
}

const useStyles = makeStyles(styles);

const Layout: React.FC = (props) => {
  const classes = useStyles()

  return (
    <React.Fragment>
      <Header id="back-to-top-anchor"/>
      <Container maxWidth="md" className={classes.layout}>
        <main>{props.children}</main>
      </Container>

      <ScrollTop {...props}>
        <Fab className={classes.fab} size="medium" aria-label="scroll back to top">
          <KeyboardArrowUpIcon />
        </Fab>
      </ScrollTop>
    </React.Fragment>
  );
};

export default Layout;
