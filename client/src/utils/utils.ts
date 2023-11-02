import { Theme, ToastOptions, ToastPosition} from 'react-toastify';
import type { MenuProps } from 'antd';

export type MenuItem = Required<MenuProps>['items'][number];
export const getItem = (
  label: React.ReactNode,
  key: React.Key,
  icon?: React.ReactNode,
  children?: MenuItem[],
  type?: 'group',
): MenuItem => {
  return {
    key,
    icon,
    children,
    label,
    type,
  } as MenuItem;
}

export const toastMSGObject = ({
    position = "top-right" as ToastPosition,
    autoClose = 2000,
    hideProgressBar = false,
    closeOnClick = true,
    pauseOnHover = true,
    draggable = true,
    progress = undefined,
    theme = "colored" as Theme,
} = {}): ToastOptions<{}> => ({
  position,
  autoClose,
  hideProgressBar,
  closeOnClick,
  pauseOnHover,
  draggable,
  progress,
  theme
})