function export_vars_for_environment {
  local environment=$1
  local env_file="crates/collab/k8s/environments/${environment}.sh"
  if [[ ! -f $env_file ]]; then
    echo "Invalid environment name '${environment}'" >&2
    exit 1
  fi
  export $(cat $env_file)
}

function target_neopilot_kube_cluster {
  if [[ $(kubectl config current-context 2> /dev/null) != do-nyc1-neopilot-1 ]]; then
    doctl kubernetes cluster kubeconfig save neopilot-1
  fi
}

function tag_for_environment {
  if [[ "$1" == "production" ]]; then
    echo "collab-production"
  elif [[ "$1" == "staging" ]]; then
    echo "collab-staging"
  else
    echo "Invalid environment name '${environment}'" >&2
    exit 1
  fi
}

function url_for_environment {
  if [[ "$1" == "production" ]]; then
    echo "https://collab.neopilot.dev"
  elif [[ "$1" == "staging" ]]; then
    echo "https://collab-staging.neopilot.dev"
  else
    echo "Invalid environment name '${environment}'" >&2
    exit 1
  fi
}
