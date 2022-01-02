import {
  createStyles
} from "@mui/styles";

const styles = () => createStyles({
  layout: {
    position: 'relative',
    boxSizing: 'border-box',
    minHeight: 'calc(-60px + 100vh)',
    paddingTop: 30,
    paddingBottom: 30,
  },
  fab: {
    background: '#000 !important',
    color: '#fff !important',
  }
})

export default styles