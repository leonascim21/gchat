import { useState } from "react";
import SignInModal from "./signInModal";
import SignUpModal from "./signupModal";

interface Props {
  hideAuthModal: () => void;
}

export default function AuthModals({ hideAuthModal }: Props) {
  const [signInVisible, setSignInVisible] = useState(true);
  const [signUpVisible, setSignUpVisible] = useState(false);

  const switchAuthForm = () => {
    setSignInVisible(!signInVisible);
    setSignUpVisible(!signUpVisible);
  };

  return (
    <div>
      {signInVisible && (
        <SignInModal
          showSignUp={switchAuthForm}
          successfullSignIn={hideAuthModal}
        />
      )}
      {signUpVisible && (
        <SignUpModal
          showSignIn={switchAuthForm}
          successfullSignUp={hideAuthModal}
        />
      )}
    </div>
  );
}
